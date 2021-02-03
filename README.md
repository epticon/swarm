# Epticon Swarm

Comprises of:

- [Telemetary](./telemetary)
- [Video Streaming](./videostream)

## Stack

Telemetary:

- `Rust`
- `Actix-web`
- `Web-socket`
- `PostgreSQL`

Videostream:

- `Nginx`

## Task

### Application: Telemetary

- [ ] Create basic folder structure using `actix-web`.
- [ ] Integrate web-sockets.
- [ ] Websocket Authentication with JWT Issues streaming token.
- [ ] Develop Application Schema

### Database

- [ ] Test database config and connection on Docker
- [ ]

### Enviroment

- [ ] Improve the telemetary docker setup (push only binary to container)
- [ ] Don't create several video stream versions for all enviroment (introduce versions only if the enviroment is set to `production-hd`). The simplest way is to create several nginx config and copy to the contianer based on specified enviroment variable.
