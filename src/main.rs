use std::env;
use chess_solver::*;

fn main() {
    let args : Vec<String> = env::args().collect();
    match args.len() {
        5 => {
            let s = Solver{moves:Knight::moves(), board_size:8, maximum_moves:6};
            let initial = Position{x:args[1].parse::<i16>().unwrap(), y:args[2].parse::<i16>().unwrap()};
            let destination = Position{x:args[3].parse::<i16>().unwrap(), y:args[4].parse::<i16>().unwrap()};
            match s.find_shortest_path(initial, destination) {
                Some(path) => {
                    println!("{:?}", path)
                }
                None => {println!("No path found")}
            }
        }
        _ => {
            println!("Usage: solver <startingX> <startingY> <destinationX> <destinationY>");
        }
    }

}

mod chess_solver {
    #[derive(Debug)]
    pub struct Movement {
        x:i16,
        y:i16
    }
    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct Position {
        pub x:i16,
        pub y:i16
    }


    pub struct Solver {
        pub moves:Vec<Movement>,
        pub board_size:i16,
        pub maximum_moves:u16
    }

    impl Solver {
        pub fn find_shortest_path(&self, initial:Position, destination:Position) -> Option<Vec<Position>> {
            for depth in 0..self.maximum_moves {
                if let Some(path) = self.find_path(&initial, &destination, depth) {
                    let mut path = path;
                    path.push(initial);
                    path.reverse();

                    return Some(path);
                }
            }
            None
        }

        fn find_path(&self, initial:&Position, destination:&Position, remaining_moves:u16) -> Option<Vec<Position>> {
    
            match remaining_moves {
                0 => {
                    match *initial == *destination {
                        true => {Some(vec!())}
                        false => {None}
                    }
                }
                _=> {
                        for current_move in &self.moves {
                            let next_x = initial.x + current_move.x;
                            let next_y = initial.y + current_move.y;
                            if next_x >= 0 && next_x < self.board_size && next_y >= 0 && next_y < self.board_size {
                            let next = Position{x : initial.x + current_move.x, y : initial.y + current_move.y};
                                if let Some(v) = self.find_path(&next, destination, remaining_moves - 1) {
                                    let mut path = v;
                                    path.push(next); 
                                    return Some(path);
                                }
                            }
                        }
                        None
                    }
                }
            }
            
        }



    pub struct Knight {}

    impl Knight {
        pub fn moves() -> Vec<Movement> {
            vec!(Movement{x:1, y:2}, Movement{x:-1, y:2}, Movement{x:1, y:-2}, Movement{x:-1, y:-2}, Movement{x:2, y:1}, 
            Movement{x:-2, y:1}, Movement{x:2, y:-1}, Movement{x:-2, y:-1}, 
            )
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_already_at_destination() {
            let solver = Solver{moves:Knight::moves(), board_size:8, maximum_moves:3};
            let solution = solver.find_shortest_path(Position{x:5, y:6}, Position{x:5, y:6});
            match solution {
                Some(path) => {
                    assert_eq!(path.len(), 1);
                    assert_eq!(path.get(0), Some(&Position{x:5, y:6}));
                }
                None => panic!()
            }
        }

        #[test]
        fn test_one_move_away() {
            let solver = Solver{moves:Knight::moves(), board_size:8, maximum_moves:3};
            let solution = solver.find_shortest_path(Position{x:4, y:4}, Position{x:5, y:6});
            match solution {
                Some(path) => {
                    assert_eq!(path.len(), 2);
                    assert_eq!(path.get(0), Some(&Position{x:4, y:4}));
                    assert_eq!(path.get(1), Some(&Position{x:5, y:6}));
                }
                None => panic!()
            }
        }

        #[test]
        fn test_too_far() {
            let solver = Solver{moves:Knight::moves(), board_size:8, maximum_moves:3};
            let solution = solver.find_shortest_path(Position{x:0, y:0}, Position{x:5, y:6});
            if let Some(_) = solution {
                panic!();
            }
        }
    }
}
