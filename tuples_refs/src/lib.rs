// Define the Student tuple struct
pub struct Student(pub u32, pub String, pub String);

// Function to get the id of the student
pub fn id(student: &Student) -> u32 {
    student.0
}

// Function to get the first name of the student
pub fn first_name(student: &Student) -> String {
    student.1.clone()
}

// Function to get the last name of the student
pub fn last_name(student: &Student) -> String {
    student.2.clone()
}
