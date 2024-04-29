// #1
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//#4
// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main(){
    // #2
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // #3
    user1.email = String::from("someone@example.com");


    let user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // #5
    let user3 = User {
        active: user2.active,
        username: user2.username,
        email: String::from("another@example.com"),
        sign_in_count: user2.sign_in_count,
    };
    let user4 = User {
        email: String::from("another@example.com"),
        ..user3
    };


}