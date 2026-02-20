# Branch Instructions
1. For explanatory prompts save the input and output to a file in prompts called {branch_name}-{YYYY-MM-DD}/{branch_name}-ai-{YYYY-MM-DD}-{iteration} 
2. Before making a change, checkout a new branch named {branch_name}-ai-{iteration}
3. Environmental variables not stored in this repo should be non-accessible for read or transmission to the public or models. They can however be used for coding purposes.

# Open-API instructions
1. Each service should stand up an endpoint that handles swagger
2. All Rest endpoints should use Utopia
3. All grpc objects that are used as pass through should be converted to something utopia can understand
4. Whenever an endpoint changes, the relevant document should
5. dental-common will store all query params
EX:
```rust
#[derive(Deserialize, ToSchema, IntoParams)]
pub struct UserKeyPathParam {
    /// The unique username of the user
    pub user: String,
}
