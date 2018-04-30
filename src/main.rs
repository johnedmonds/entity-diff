use std::cmp::min;
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;

#[derive(Copy, Debug, Eq, PartialEq)]
pub enum Edit<'a, T: 'a + Eq> {
    Insert(&'a T),
    Delete,
    InsertAndDelete(&'a T),
    Keep
}

impl<'a, T: 'a + Eq> Clone for Edit<'a, T> {
    fn clone(&self) -> Self {
        match self {
            Edit::Insert(t) => Edit::Insert(t),
            Edit::Delete => Edit::Delete,
            Edit::Keep => Edit::Keep,
            Edit::InsertAndDelete(t) => Edit::InsertAndDelete(t)
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
        for j in 1..b.len() + 1 {
            let deletion = &grid[i - 1][j];
            let insertion = &grid[i][j - 1];
            let insertion_and_deletion = &grid[i - 1][j - 1];
            let deletion_cost = 1 + deletion.borrow().cost;
            let insertion_cost = 1 + insertion.borrow().cost;

            let insertion_and_deletion_cost = if a[i - 1] == b[j - 1] {
                insertion_and_deletion.borrow().cost
            } else {
                2 + insertion_and_deletion.borrow().cost
            };

            let min_cost = min(min(insertion_cost, deletion_cost), insertion_and_deletion_cost);

            let mut current_grid_square = grid[i][j].borrow_mut();

            if insertion_cost == min_cost {
                current_grid_square.cost = insertion_cost;
                current_grid_square.from = Some(Rc::downgrade(&insertion));
                current_grid_square.edit = Edit::Insert(&b[j - 1]);
            } else if deletion_cost == min_cost {
                current_grid_square.cost = deletion_cost;
                current_grid_square.from = Some(Rc::downgrade(&deletion));
                current_grid_square.edit = Edit::Delete;
            } else {
                current_grid_square.cost = insertion_and_deletion_cost;
                current_grid_square.from = Some(Rc::downgrade(&insertion_and_deletion));
                current_grid_square.edit = if a[i - 1] == b[j - 1] {
                    Edit::Keep
                } else {
                    Edit::InsertAndDelete(&b[j - 1])
                }
            }
        }
    }

    return grid[a.len()][b.len()].borrow().path();
}

fn main() {
    let a = "abc";
    let b = "bcd";
    let a_vec: Vec<char> = a.chars().collect();
    let b_vec: Vec<char> = b.chars().collect();

    let diff_vec = diff(&a_vec, &b_vec);
    println!("{:?}", diff_vec);
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
    fn insert() {
        let c = 'd';
        assert_diff("abc", "bcd", vec![Edit::Keep, Edit::Keep, Edit::Insert(&c)]);
    }
}