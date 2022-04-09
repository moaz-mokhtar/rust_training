use crate::{
    db::DbPool,
    entity::user::{Reset, User, UserDTO, UserRegisterationRequest, UserToken},
    schema::{reset::dsl::reset as reset_schema, user_token::dsl::user_token, users::dsl::users},
    MyError,
};

use bcrypt::verify;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use uuid::Uuid;

impl User {
    pub async fn insert(
        incoming: UserRegisterationRequest,
        pool: &DbPool,
    ) -> Result<UserDTO, MyError> {
        let hashed_password = hash(&incoming.password, DEFAULT_COST).unwrap();
        let new_user = User {
            id: Uuid::new_v4(),
            first_name: incoming.first_name,
            last_name: incoming.last_name,
            email: incoming.email,
            password: hashed_password,
        };
        let connection = pool.get().unwrap();
        let feedback: User = diesel::insert_into(crate::schema::users::dsl::users)
            .values(&new_user)
            .get_result(&connection)
            .unwrap();

        Ok(feedback.as_dto())
    }

    pub async fn find_by_id(incoming_id: Uuid, pool: &DbPool) -> Result<UserDTO, MyError> {
        let connection = pool.get().unwrap();
        let feedback: User = users.find(incoming_id).first(&connection).unwrap();

        Ok(feedback.as_dto())
    }

    pub async fn find_by_email(incoming_email: String, pool: &DbPool) -> Result<User, MyError> {
        use crate::schema::users::email;

        let connection = pool.get().unwrap();
        let feedback: User = users
            .filter(email.eq(incoming_email))
            .first(&connection)
            .unwrap();

        Ok(feedback)
    }

    pub async fn get_all(pool: &DbPool) -> Result<Vec<User>, MyError> {
        let connection = pool.get().unwrap();
        let users_list = users.load::<User>(&connection).unwrap();

        Ok(users_list)
    }

    pub async fn authenticate_by_email(
        email: String,
        incoming_password: String,
        pool: &DbPool,
    ) -> Result<User, MyError> {
        let user = User::find_by_email(email, &pool).await.unwrap();

        if verify(incoming_password, &user.password).unwrap() {
            Ok(user)
        } else {
            Err(MyError::General {
                desc: "unauthenticated".to_string(),
            })
        }
    }

    pub async fn update_password(
        incoming_id: Uuid,
        incoming_password: String,
        pool: &DbPool,
    ) -> Result<(), MyError> {
        use crate::schema::users::dsl::id;
        use crate::schema::users::dsl::password;
        let hashed_password = hash(&incoming_password, DEFAULT_COST).unwrap();

        let connection = pool.get().unwrap();
        diesel::update(users)
            .filter(id.eq(incoming_id))
            .set(password.eq(hashed_password))
            .execute(&connection)
            .map_err(|e| MyError::General {
                desc: format!("{}", e),
            })
            .unwrap();

        Ok(())
    }
}

impl UserToken {
    pub async fn insert(incoming: UserToken, pool: &DbPool) -> Result<UserToken, MyError> {
        let connection = pool.get().unwrap();
        let user: User = users.find(&incoming.user_id).first(&connection).unwrap();

        let user_token_object: UserToken =
            diesel::insert_into(crate::schema::user_token::dsl::user_token)
                .values(&incoming)
                .get_result(&connection)
                .unwrap();

        Ok(user_token_object)
    }

    /// Find a token related to a user_id
    pub async fn find_by_token(
        incoming_token: String,
        incoming_user_id: Uuid,
        pool: &DbPool,
    ) -> Result<UserToken, MyError> {
        use crate::schema::user_token::token;
        use crate::schema::user_token::user_id;

        let connection = pool.get().unwrap();
        let user_token_object = user_token
            .filter(user_id.eq(incoming_user_id))
            .filter(token.eq(incoming_token))
            .first(&connection)
            .map_err(|e| MyError::General {
                desc: format!("{}", e),
            })
            .unwrap();

        Ok(user_token_object)
    }

    /// Delete user_token by refresh_token
    /// This is used when a user logs out
    /// It will delete the user_token from the database
    pub async fn delete(incoming_token: String, pool: &DbPool) -> Result<(), MyError> {
        use crate::schema::user_token::token;
        use crate::schema::user_token::user_id;

        let connection = pool.get().unwrap();
        diesel::delete(user_token.filter(token.eq(incoming_token)))
            .execute(&connection)
            .map_err(|e| MyError::General {
                desc: format!("{}", e),
            })
            .unwrap();

        Ok(())
    }
}

impl Reset {
    pub async fn insert(
        incoming_email: String,
        incoming_token: String,
        pool: &DbPool,
    ) -> Result<Reset, MyError> {
        let connection = pool.get().unwrap();

        let reset = Reset {
            token: incoming_token,
            email: incoming_email,
        };

        let reset_object = diesel::insert_into(crate::schema::reset::dsl::reset)
            .values(&reset)
            .get_result(&connection)
            .map_err(|e| MyError::General {
                desc: format!("{}", e),
            })
            .unwrap();

        Ok(reset_object)
    }

    /// Find a Reset object related to a token
    pub async fn find_by_token(incoming_token: String, pool: &DbPool) -> Result<Reset, MyError> {
        use crate::schema::reset::token;

        let connection = pool.get().unwrap();
        let reset_object = reset_schema
            .filter(token.eq(incoming_token))
            .first(&connection)
            .map_err(|e| MyError::General {
                desc: format!("{}", e),
            })
            .unwrap();

        Ok(reset_object)
    }
}
