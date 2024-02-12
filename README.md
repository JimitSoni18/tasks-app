# This is a task app implementation and includes user registration, authentication, authorization and Task CRUD application.


> This app also serves as a reference implementation for a lot of my server applications boilerplate, as it includes most of the boilerplate and database models, as well as json-rpc API

---

This app utilizes a json-rpc architecture, hence it is scalable and modular for further development

```bash
# Start postgresql server podman image:
podman run --rm --name pg -p 5432:5432 \
	-e POSTGRES_PASSWORD=welcome \
	postgres

# In another terminal (tab) run psql:
podman exec -it -u postgres pg psql
```
