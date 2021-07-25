SRBIA (Simple Rust Blog in Action)


TODO:
===============================================================================
- ~~Test db methods~~.`
- handle url resource GET /usrs/{id}
- `~~create `GET /posts/new` resource.`~~
- ~~Signup form template should render errors on current/same signup page~~
- ~~Create login page~~
- Hash user password when signing up
- ~~Issue #01: index page isn't being loaded. Error: "App is not configured"~~
- ~~Create handler for `GET /users/{id}`~~
- ~~Issue #02: `GET /users/{id``}` returns password field when it shouldn't~~
- Create user session when user logs in / signs up
- Create `authenticate` helper to verify that new user doesn't already exist
- ~~Create handler for `Get /login` ~~
- Create custom error templates
- Create UML diagram
- ~~Issue #03: Redirecting to `/index` not working (no connection?)~~ 
- ~~Create SQL table `sessions` .~~
- ~~Create `db::create_user_session`  which creates a new user session.~~ 
- Create handler for resource `GET /sessions/{user_id}`
- Create handler for resource `POST /sessions/{user_id}`
- Implement password verification for `Auth::authenticate`
- ~~Issue #01: index page isn't being loaded. Error: "App is not configured"~~
- ~~Create handler for `GET /users/{id}`~~
- ~~Issue #02: `GET /users/{id``}` returns password field when it shouldn't~~ 
- Create handler for `logout` functionality
- Create App state to store data
- ~~Issue #04: Login page keeps returning "UserNotFound" error despite user existing in
    db~~ 
- Refactor `Auth` and `Verify` traits (and maybe other form-related traits)
- Create  `forms` module
- Create session id from `rng`.
- Add login/logout methods to `Auth` trait?
