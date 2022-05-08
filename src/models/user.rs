use uuid::Uuid;
use chrono::NaiveDateTime;
use serde::{Serialize,Deserialize};
use validator_derive::Validate;
use sqlx::Database;


#[derive(Deserialize,Serialize,Debug ,sqlx::FromRow)]
pub struct MessageBundle {
    pub uuid : Uuid,
    pub username : String,
    pub cipherText : Vec<u8>,
    pub key : Vec<u8>,
    pub nonce :String,
    pub conversationName : String,
    pub date : String,
}


#[derive(Deserialize,Serialize,Debug ,sqlx::FromRow)]
pub struct MessageBundleStringify {
    pub uuid : Uuid,
    pub username : String,
    pub cipherText : String,
    pub key :String,
    pub nonce :String,
    pub conversationName : String,
    pub date : String,
}



#[derive(Debug,Serialize, sqlx::FromRow)]
pub struct User {
    pub id :Uuid,
    //pub username: Option<String>,
    pub email: String,
    //#[serde(skip_serializing)] //to not show the password hash
    //pub password_hash : Option<String>,
    pub full_name: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at : NaiveDateTime,
    pub updated_at : NaiveDateTime,
 

}

#[derive(Debug,Serialize,Deserialize, sqlx::FromRow)]
pub struct SelectFriendAdd {
    pub id_user : String,
    pub friend: String,
    pub created_at : NaiveDateTime,
}

#[derive(Debug,Serialize,Deserialize, sqlx::FromRow)]
pub struct FriendAdd {
    pub id_user : Uuid,
    pub friend: Uuid,
    pub created_at : NaiveDateTime,
}

#[derive(Debug,Serialize,Deserialize, sqlx::FromRow)]
pub struct SearchUsers {
    pub id : Uuid,
    pub full_name: String,
}

#[derive(Debug,Serialize,Deserialize, sqlx::FromRow)]
pub struct YourFriend {
    pub friend : String,
}

#[derive(Debug,Serialize,Deserialize, sqlx::FromRow)]
pub struct KillFriendship {
    pub id2 : Uuid,
    
}

#[derive(Debug,Serialize,Deserialize, sqlx::FromRow)]
pub struct FindFullName {
    pub full_name : String,
}
#[derive(Debug,Serialize,Deserialize, sqlx::FromRow)]
pub struct QueryAddFriend {
    pub friend_id_to_add : Uuid,
    pub friend_full_name : String,
}

// #[derive(Debug,Serialize, sqlx::FromRow)]
// pub struct Userpass {
//     pub id :Uuid,
//     pub username: String,
//     //pub email: String,
//     #[serde(skip_serializing)] //to not show the password hash
//     pub password_hash : String,
//     //pub full_name: String,
//     pub created_at : NaiveDateTime,
//     pub updated_at : NaiveDateTime,
 

// }

//validate the input fields for new users and update users 
// looking for new ideas
    //  - strongre password
    //  - newusers



// #[derive(Debug,Deserialize, Validate)]
// pub struct NewUser {
//     #[validate(length(min = 3))]
//     pub username : String,
//     #[validate(email)]
//     pub email : String,
//     #[validate(length(min = 7))]
//     pub password : String,
//     #[validate(length(min = 1))]
//     pub full_name : String,
// }

// #[derive(Debug,Deserialize, Validate)]
// pub struct UpdateProfile {
//     pub full_name:Option<String>,
//     pub bio:Option<String>,
//     pub image:Option<String>,
// }