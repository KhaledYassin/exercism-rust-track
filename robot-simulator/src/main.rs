use robot_simulator::*;
fn main() {
    let robot = Robot::new(0, 0, Direction::North).instructions("LAAARALA");
    assert_eq!((-4, 1), robot.position());
    assert_eq!(&Direction::West, robot.direction());
}