enum Direction {
    Up,
    Down, 
    Left,
    Right,
}

fn move_player (dir:Direction){
    match dir{
        Direction::Up => println!("move upward"),
        Direction::Down => println!("move upward"),
        Direction::Left => println!("move upward"),
        Direction::Right => println!("move upward"),
    }
}
fn main(){
    let dir = Direction::Left;
    move_player(dir); 
}