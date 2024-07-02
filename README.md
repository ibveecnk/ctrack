**ctrack**

table of contents
- [architecture](#architecture)
- [running the components](#running-the-components)
  - [with nix](#with-nix)
  - [manual](#manual)


## architecture
The stack is split into two areas of concern - the `frontend` presenting data and the `backend` handling business logic. The module `shared` interfaces between the two by providing safe interfaces between the two.

## running the components
### with nix
1. enter the devshell: `nix develop`
2. run the command `run-frontend`, `run-backend`
3. profit
### manual
```sh
cd frontend && trunk serve
```
```sh
cd backend && cargo run
```