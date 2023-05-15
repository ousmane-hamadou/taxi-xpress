podman run --name xpress-http\
 -v xpress_data:/var/lib/postgresql/data\
 -p 5432:5432\
 -e POSTGRES_PASSWORD=postgres\
 -d --rm postgres
