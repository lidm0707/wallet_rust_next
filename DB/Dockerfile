# Use the official PostgreSQL image as the base
FROM postgres:latest

# Set environment variables for PostgreSQL
ENV POSTGRES_USER=myuser
ENV POSTGRES_PASSWORD=mypassword
ENV POSTGRES_DB=mydatabase

# Optionally, copy initialization SQL scripts to the container
# COPY init.sql /docker-entrypoint-initdb.d/

# Expose the PostgreSQL default port
EXPOSE 5432

# Use the default entrypoint provided by the PostgreSQL image
