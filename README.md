# Subscriber Project

This project is designed to help manage subscribers. 

### Using Docker Compose

Here is an example of how to use Docker Compose with this project.

```yaml
version: '3.8'

services:
  subscriber:
    image: ghcr.io/sparx-foundation/subscriber:latest
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "8080:8080"
    environment:
      SMTP_NAME=your_smtp_name
      SMTP_PASS=your_smtp_password
      SMTP_HOST=your_smtp_host
      DATABASE_URL=your_database_url
      SERVER_PORT=your_server_port
      ALLOWED_ORIGINS=origin1,origin2,origin3

  # Add any additional services here (like psotgres)
  postgres:
    image: postgres:latest
    environment:
      POSTGRES_DB: your_database_name
      POSTGRES_USER: your_database_user
      POSTGRES_PASSWORD: your_database_password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    ports:
      - "5432:5432"

volumes:
  postgres_data:
```

### Running the Application

Once the containers are up and running, you can access the application by navigating to `http://localhost:8080` in your web browser.

### Contributing

If you would like to contribute to this project, please fork the repository and submit a pull request.

### License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

