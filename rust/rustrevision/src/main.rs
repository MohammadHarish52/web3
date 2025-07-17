enum UserRole {
    Admin,
    Editor,
    Viewer,
}

fn describe(role: UserRole) {
    match role {
        UserRole::Admin => {
            println!("User is Admin");
        }
        UserRole::Editor => {
            println!("USer is editor")
        }
        UserRole::Viewer => {
            println!("user is viewer")
        }
    }
}

fn main() {
    let user1 = UserRole::Admin;
    let user2 = UserRole::Editor;

    describe(user1);
    describe(user2);
}
