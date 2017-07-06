use super::*;

#[get("/")]
pub fn index(user: guards::User) -> Template {   // Try this / route first, skip if user guard not satisfied
    let context = authentication_contexts::IndexContext{
        name: "Jack".to_string(),
        parent: "layout".to_string(),
        title: "Index".to_string()
    };
    println!("User id {}", user.0);

    Template::render("index", &context)
}

#[get("/", rank = 2)]
fn index_unauthorised() -> Redirect {
    Redirect::to("/login")
}

#[get("/register")]
pub fn register() -> Template {
    let context = authentication_contexts::RegisterContext{
        title: "Registration".to_string(),
        errors: None,
        parent: "layout".to_string()
    };

    Template::render("register", &context)
}

#[post("/register", format = "application/x-www-form-urlencoded", data = "<registration>")]
pub fn register_post(registration: Form<authentication_forms::RegistrationForm>) -> Redirect {
    let registration_form = registration.get();

    // todo Handle errors, verification etc
    let connection = user_repository::establish_connection();

    let result = user_repository::create_user(
        &connection, &registration_form.user_name, &registration_form.password
    );

    match result {
        Some(_) => Redirect::to("/login"),
        None => Redirect::to("/register")
    }
}

#[get("/login")]
fn login_user(_user: guards::User) -> Redirect {
    Redirect::to("/")
}

#[get("/login", rank=2)]
pub fn login() -> Template {
    let context = authentication_contexts::LoginContext{
        title: "Login".to_string(),
        parent: "layout".to_string()
    };

    Template::render("login", &context)
}

#[post("/login", format = "application/x-www-form-urlencoded", data = "<login>")]
pub fn login_post(mut cookies: Cookies, login: Form<authentication_forms::LoginForm>) -> Flash<Redirect> {
    let login_form = login.get();

    // Get user
    let connection = user_repository::establish_connection();
    let user = user_repository::get_user_by_username(&connection, &login_form.user_name);

    match user {
        Some(user) => {
            match verify(&login_form.password, &user.password) {
                Ok(_) => {
                    cookies.add_private(Cookie::new("user_id", user.id.to_string()));
                    Flash::success(Redirect::to("/"), "Successfully logged in")
                },
                Err(_) => Flash::error(Redirect::to("/login"), "Incorrect Password"),
            }
        },
        None => Flash::error(Redirect::to("/login"), "Incorrect Username")
    }
}

#[get("/logged_out")]
pub fn logout(mut cookies: Cookies) -> Template {
    cookies.remove_private(Cookie::named("user_id"));

    let context = authentication_contexts::LogoutContext{
        title: "Logout".to_string(),
        parent: "layout".to_string()
    };

    Template::render("logged_out", &context)
}