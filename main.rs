mod teacher;
mod syllabus;
mod timetable;

fn main() {
    teacher::maths();
    teacher::physics();
    teacher::chemistry();
    teacher::computer();
    println!("\n");
    syllabus::maths();
    syllabus::physics();
    syllabus::chemistry();
    syllabus::computer();
    println!("\n");
    timetable::maths();
    timetable::computer(); 
    timetable::physics();
    timetable::chemistry();
}