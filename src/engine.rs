use crate::{
    db::DbPool,
    entity::user::{User, UserRegisteration},
    schema::users::dsl::users,
};

use bcrypt::verify;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use uuid::Uuid;
type DbError = Box<dyn std::error::Error + Send + Sync>;

// impl User {
//     pub async fn get_all(connection: &MysqlConnection) -> Result<Vec<User>, DbError> {
//         use crate::schema::users::dsl::users;
//         let _users = users.load::<User>(connection)?;

//         Ok(_users)
//     }
// }

impl User {
    pub async fn insert(incoming: UserRegisteration, pool: &DbPool) -> Result<User, DbError> {
        let hashed_password = hash(&incoming.password, DEFAULT_COST)?;

        let new_user = User {
            id: Uuid::new_v4(),
            first_name: incoming.first_name,
            last_name: incoming.last_name,
            email: incoming.email,
            username: incoming.username,
            password: hashed_password,
        };
        let connection = pool.get()?;
        let feedback = diesel::insert_into(crate::schema::users::dsl::users)
            .values(&new_user)
            .get_result(&connection)?;

        Ok(feedback)
    }

    pub async fn find_by_id(incoming_id: Uuid, pool: &DbPool) -> Result<User, DbError> {
        let connection = pool.get()?;
        let feedback = users.find(incoming_id).first(&connection)?;

        Ok(feedback)
    }

    pub async fn find_by_username(
        incoming_username: String,
        pool: &DbPool,
    ) -> Result<User, DbError> {
        use crate::schema::users::username;

        let connection = pool.get()?;
        let feedback = users
            .filter(username.eq(incoming_username))
            .first(&connection)?;

        Ok(feedback)
    }

    pub async fn get_all(pool: &DbPool) -> Result<Vec<User>, DbError> {
        let connection = pool.get()?;
        let _users = users.load::<User>(&connection)?;

        Ok(_users)
    }

    pub async fn authenticate(
        username: String,
        incoming_password: String,
        pool: &DbPool,
    ) -> Result<bool, DbError> {
        let user = User::find_by_username(username, &pool).await?;
        let is_authenticated = verify(incoming_password, &user.password)?;

        Ok(is_authenticated)
    }
}
