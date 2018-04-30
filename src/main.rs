use std::cmp::min;
use std::rc::Rc;
use std::cell::RefCell;
use std::rc::Weak;

#[derive(Copy)]
enum Edit<'a, T: 'a + Eq> {
    Insert(&'a T),
    Delete,
    Keep
}

impl<'a, T: 'a + Eq> Clone for Edit<'a, T> {
    fn clone(&self) -> Self {
        match self {
            Edit::Insert(t) => Edit::Insert(t),
            Edit::Delete => Edit::Delete,
            Edit::Keep => Edit::Keep
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

fn diff<'a, T: Eq>(a: &Vec<T>, b: &Vec<T>) -> Vec<Edit<'a, T>> {
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

            let insertion_and_deletion_cost = if a[i] == b[j] {
                insertion_and_deletion.borrow().cost
            } else {
                2 + insertion_and_deletion.borrow().cost
            };

            let min_cost = min(min(insertion_cost, deletion_cost), insertion_and_deletion_cost);

            let mut current_grid_square = grid[i][j].borrow_mut();

            if insertion_cost == min_cost {
                current_grid_square.cost = insertion_cost;
                current_grid_square.from = Some(Rc::downgrade(&insertion));
            } else if deletion_cost == min_cost {
                current_grid_square.cost = deletion_cost;
                current_grid_square.from = Some(Rc::downgrade(&deletion));
            } else {
                current_grid_square.cost = insertion_and_deletion_cost;
                current_grid_square.from = Some(Rc::downgrade(&insertion_and_deletion));
            }
        }
    }

    return grid[a.len()][b.len()].borrow().path();
}

fn main() {
    println!("Hello, world!");
}
