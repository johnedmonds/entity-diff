use std::cmp::min;
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;

/// A transformation to an index in a vector.
/// 
/// There will be an edit for each character in the input string. Keep means keep the entity in the output, Delete means remove it, and Insert means add something at this index.
/// 
/// When we say "current position," consider copying the original string into an output vector, current position is the end of that vector.
#[derive(Copy, Debug, Eq, PartialEq)]
pub enum Edit<'a, T: 'a + Eq> {
    /// Add an element at the current position.
    Insert(&'a T),
    /// Delete the element from the input vector.
    Delete,
    /// Keep the original character in the output.
    Keep
}

impl<'a, T: 'a + Eq> Clone for Edit<'a, T> {
    fn clone(&self) -> Self {
        match self {
            Edit::Insert(t) => Edit::Insert(t),
            Edit::Delete => Edit::Delete,
            Edit::Keep => Edit::Keep,
        }
    }
}

struct GridSquare<'a, T: 'a + Eq> {
    cost: i32,
    from: Option<Weak<RefCell<GridSquare<'a, T>>>>,
    edit: Edit<'a, T>
}

impl<'a, T: 'a + Eq> GridSquare<'a, T> {
    fn path(&self) -> Vec<Edit<'a, T>> {
        self.from.as_ref().map(|from| {
            let rc = from.upgrade().unwrap();
            let mut path: Vec<Edit<'a, T>> = rc.borrow().path();
            path.push(self.edit.clone());
            return path;
        }).unwrap_or(Vec::new())
    }
}

/// Returns the edits required to change `a` into `b`
/// 
/// Edits are applied to each character in `a`. See `Edit` to determine what each type of Edit does.
pub fn diff<'a, T: Eq>(a: &'a Vec<T>, b: &'a Vec<T>) -> Vec<Edit<'a, T>> {
    let grid: Vec<Vec<Rc<RefCell<GridSquare<'a, T>>>>> = (0..a.len() + 1).map(|_a| {
        return (0..b.len() + 1).map(|_b| {
            return Rc::new(RefCell::new(GridSquare{
                cost: 0,
                from: None,
                edit: Edit::Keep
            }));
        }).collect::<Vec<Rc<RefCell<GridSquare<'a, T>>>>>();
    }).collect::<Vec<Vec<Rc<RefCell<GridSquare<'a, T>>>>>>();

    for i in 1..a.len() + 1 {
        let mut grid_square = grid[i][0].borrow_mut();
        grid_square.cost = i as i32;
        grid_square.from = Some(Rc::downgrade(&grid[i - 1][0]));
        grid_square.edit = Edit::Delete;
    }

    for j in 1..b.len() + 1 {
        let mut grid_square = grid[0][j].borrow_mut();
        grid_square.cost = j as i32;
        grid_square.from = Some(Rc::downgrade(&grid[0][j - 1]));
        grid_square.edit = Edit::Insert(&b[j - 1]);
    }

    for i in 1..a.len() + 1 {
        for j in 1..b.len() + 1 {
            let deletion_cell = &grid[i - 1][j];
            let insertion_cell = &grid[i][j - 1];
            let keep_cell = &grid[i - 1][j - 1];
            let deletion_cost = 1 + deletion_cell.borrow().cost;
            let insertion_cost = 1 + insertion_cell.borrow().cost;
            let keep_cost = keep_cell.borrow().cost;

            let min_cost = if a[i - 1] == b[j - 1] {
                min(min(insertion_cost, deletion_cost), keep_cost)
            } else {
                min(insertion_cost, deletion_cost)
            };

            let mut current_grid_square = grid[i][j].borrow_mut();

            if insertion_cost == min_cost {
                current_grid_square.cost = insertion_cost;
                current_grid_square.from = Some(Rc::downgrade(insertion_cell));
                current_grid_square.edit = Edit::Insert(&b[j - 1]);
            } else if deletion_cost == min_cost {
                current_grid_square.cost = deletion_cost;
                current_grid_square.from = Some(Rc::downgrade(deletion_cell));
                current_grid_square.edit = Edit::Delete;
            } else {
                current_grid_square.cost = keep_cost;
                current_grid_square.from = Some(Rc::downgrade(keep_cell));
                current_grid_square.edit = Edit::Keep;
            }
        }
    }

    return grid[a.len()][b.len()].borrow().path();
}

#[cfg(test)]
mod tests {
    use super::*;
    fn assert_diff(a: &str, b: &str, edits: Vec<Edit<char>>) {
        let a_vec: Vec<char> = a.chars().collect();
        let b_vec: Vec<char> = b.chars().collect();
        assert_eq!(edits, diff(&a_vec, &b_vec));
    }

    #[test]
    fn insert_start() {
        let c = 'a';
        assert_diff("b", "ab", vec![Edit::Insert(&c), Edit::Keep]);
    }

    #[test]
    fn insert_mid() {
        let c = 'b';
        assert_diff("ac", "abc", vec![Edit::Keep, Edit::Insert(&c), Edit::Keep]);
    }

    #[test]
    fn insert_end() {
        let c = 'b';
        assert_diff("a", "ab", vec![Edit::Keep, Edit::Insert(&c)]);
    }

    #[test]
    fn delete_start() {
        assert_diff("ab", "b", vec![Edit::Delete, Edit::Keep]);
    }

    #[test]
    fn delete_mid() {
        assert_diff("abc", "ac", vec![Edit::Keep, Edit::Delete, Edit::Keep]);
    }

    #[test]
    fn delete_end() {
        assert_diff("ab", "a", vec![Edit::Keep, Edit::Delete]);
    }

    #[test]
    fn change_start() {
        let c = 'c';
        assert_diff("ab", "cb", vec![Edit::Delete, Edit::Insert(&c), Edit::Keep]);
    }

    #[test]
    fn change_mid() {
        let c = 'd';
        assert_diff("abc", "adc", vec![Edit::Keep, Edit::Delete, Edit::Insert(&c), Edit::Keep]);
    }

    #[test]
    fn change_end() {
        let c = 'c';
        assert_diff("ab", "ac", vec![Edit::Keep, Edit::Delete, Edit::Insert(&c)]);
    }
}