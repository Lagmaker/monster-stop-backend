use crate::models::{CreateDrink, Drink, UpdateDrink};
use actix_web::{HttpResponse, Responder, web};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

// Handlers for the drinks
//     GET /drinks - list all drinks
//     GET /drinks/{id} - get a single drink by ID
//     POST /drinks - create a new drink
//     PUT /drinks/{id} - update an existing drink
//     DELETE /drinks/{id} - delete a drink

pub async fn list_drinks(db_pool: web::Data<Pool<Postgres>>) -> impl Responder {
    let drinks = sqlx::query_as::<_, Drink>("SELECT * FROM drinks ORDER BY created_at DESC")
        .fetch_all(db_pool.get_ref())
        .await;

    match drinks {
        Ok(drinks) => HttpResponse::Ok().json(drinks),
        Err(e) => {
            eprintln!("Error listing drinks: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_drink(
    path: web::Path<Uuid>,
    db_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let id = path.into_inner();
    let drink = sqlx::query_as::<_, Drink>("SELECT * FROM drinks WHERE id = $1")
        .bind(id)
        .fetch_one(db_pool.get_ref())
        .await;

    match drink {
        Ok(drink) => HttpResponse::Ok().json(drink),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("Drink not found"),
        Err(e) => {
            eprintln!("Error fetching drink: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn create_drink(
    db_pool: web::Data<Pool<Postgres>>,
    new_drink: web::Json<CreateDrink>,
) -> impl Responder {
    let result = sqlx::query_as::<_, Drink>(
        "INSERT INTO drinks (name, brand, caffeine_content, sugar_content, price)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *",
    )
    .bind(&new_drink.name)
    .bind(&new_drink.brand)
    .bind(new_drink.caffeine_content)
    .bind(new_drink.sugar_content)
    .bind(new_drink.price)
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(drink) => HttpResponse::Created().json(drink),
        Err(e) => {
            eprintln!("Error inserting drink: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update_drink(
    path: web::Path<Uuid>,
    db_pool: web::Data<Pool<Postgres>>,
    updates: web::Json<UpdateDrink>,
) -> impl Responder {
    let id = path.into_inner();

    // Fetch the current drink
    let current_drink = sqlx::query_as::<_, Drink>("SELECT * FROM drinks WHERE id = $1")
        .bind(id)
        .fetch_one(db_pool.get_ref())
        .await;

    let current_drink = match current_drink {
        Ok(d) => d,
        Err(sqlx::Error::RowNotFound) => return HttpResponse::NotFound().body("Drink not found"),
        Err(e) => {
            eprintln!("Error fetching drink to update: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Merge the updates
    let name = updates.name.clone().unwrap_or(current_drink.name);
    let brand = updates.brand.clone().unwrap_or(current_drink.brand);
    let caffeine_content = updates
        .caffeine_content
        .unwrap_or(current_drink.caffeine_content);
    let sugar_content = updates.sugar_content.unwrap_or(current_drink.sugar_content);
    let price = updates.price.unwrap_or(current_drink.price);

    let updated_drink = sqlx::query_as::<_, Drink>(
        "UPDATE drinks SET
            name = $1,
            brand = $2,
            caffeine_content = $3,
            sugar_content = $4,
            price = $5
         WHERE id = $6
         RETURNING *",
    )
    .bind(name)
    .bind(brand)
    .bind(caffeine_content)
    .bind(sugar_content)
    .bind(price)
    .bind(id)
    .fetch_one(db_pool.get_ref())
    .await;

    match updated_drink {
        Ok(d) => HttpResponse::Ok().json(d),
        Err(e) => {
            eprintln!("Error updating drink: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete_drink(
    path: web::Path<Uuid>,
    db_pool: web::Data<Pool<Postgres>>,
) -> impl Responder {
    let id = path.into_inner();
    let result = sqlx::query("DELETE FROM drinks WHERE id = $1")
        .bind(id)
        .execute(db_pool.get_ref())
        .await;

    match result {
        Ok(r) => {
            if r.rows_affected() == 0 {
                HttpResponse::NotFound().body("Drink not found")
            } else {
                HttpResponse::NoContent().finish()
            }
        }
        Err(e) => {
            eprintln!("Error deleting drink: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
