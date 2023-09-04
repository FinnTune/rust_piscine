fn main() {
    let phrase1 = "Hello my dear!";
    let phrase2 = "Hello my dear!";
    let phrase3 = String::from("Hello my dear!");
    let phrase4 = String::from("Hello my dear!");
    let phrase5 = b"Hello my dear!";
    // let mut phrase6 = b"Hello my dear!";
    // let phrase7 = b"Hello my dear!".to_vec();
    // let mut phrase8 = b"Hello my dear!".to_vec();
    // let phrase9 = b"Hello my dear!".to_vec().into_boxed_slice();
    // let mut phrase10 = b"Hello my dear!".to_vec().into_boxed_slice();
    // let phrase11 = b"Hello my dear!".to_vec().into_boxed_slice().into_vec();
    // let mut phrase12 = b"Hello my dear!".to_vec().into_boxed_slice().into_vec();
    // let phrase13 = b"Hello my dear!".to_vec().into_boxed_slice().into_vec().into_boxed_slice();

    let phrase12 = r#"Hello World""#;

    println!("phrase1: {}", phrase1);
    println!("phrase2: {}", phrase2);
    println!("phrase3: {}", phrase3);
    println!("phrase4: {}", phrase4);
    println!("phrase5: {:?}", phrase5);
    println!("phrase12: {}", phrase12);
}
