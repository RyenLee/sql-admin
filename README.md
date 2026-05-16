# LiteAdmin

A lightweight, modern SQL database administration tool built with Rust. LiteAdmin provides a web-based interface for managing database connections, executing SQL queries, and exploring database structures across multiple database types.

## Features

- **Multi-Database Support**: Connect to PostgreSQL, MySQL, and SQLite databases
- **Connection Management**: Create, edit, test, and delete database connections
- **SQL Query Editor**: Execute SQL queries with syntax highlighting
- **Table Structure Viewer**: Browse table definitions, columns, indexes, and DDL
- **Query History**: Track and reuse past SQL queries
- **Query Bookmarks**: Save favorite queries for quick access
- **Dark Mode**: Full dark theme support
- **Database Tools**: Import/export data, SQL formatting, data comparison
- **Responsive UI**: Modern, responsive interface built with TailwindCSS

## Tech Stack

| Layer      | Technology                      |
| ---------- | ------------------------------- |
| Frontend   | Leptos (Rust WASM), TailwindCSS |
| Backend    | Axum (Rust), SQLx               |
| Database   | PostgreSQL, MySQL, SQLite       |
| Build Tool | Cargo, Vite                     |

## Prerequisites

- **Rust 1.95+** - [Install Rust](https://rustup.rs/)
- **Node.js 26+** (optional, for frontend development)
- **Database Drivers**:
  - PostgreSQL: `libpq` development libraries
  - MySQL: `mysql-client` development libraries
  - SQLite: Built-in support

### Installing Database Drivers

**macOS (Homebrew)**

```bash
brew install postgresql mysql sqlite
```

**Ubuntu/Debian**

```bash
sudo apt-get install libpq-dev libmysqlclient-dev libsqlite3-dev
```

**Windows**
Install via [vcpkg](https://github.com/microsoft/vcpkg) or use pre-built binaries.

## Installation

### Build from Source

1. Clone the repository:

```bash
git clone https://github.com/RyenLee/sql-admin.git
cd sql-admin
cp .env.example .env
```

2. Build the backend:

```bash
cargo build --release -p sql-admin-backend
```

3. Build the frontend (requires wasm target):

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release -p sql-admin-frontend
```

4. Run the application:

```bash
# Start the backend server
cargo run -p sql-admin-backend

# The frontend will be served at http://localhost:3000
```

### Development Mode

For active development with hot reload:

```bash
# Terminal 1: Start the backend
cargo run -p sql-admin-backend

# Terminal 2: Start the frontend dev server
cd crates/frontend
npm install
npm run dev:css
trunk serve --port 8080
```

## Configuration

### Environment Variables

| Variable       | Description                             | Default               |
| -------------- | --------------------------------------- | --------------------- |
| `DATABASE_URL` | SQLite database for storing connections | `./data/sql_admin.sqlite3` |
| `SERVER_HOST`  | Backend server host                     | `0.0.0.0`             |
| `SERVER_PORT`  | Backend server port                     | `3000`                |
| `RUST_LOG`     | Logging level                           | `info`                |

### Creating Initial Admin

On first startup, create an admin user via the web interface at `/connections`.

## Usage

### Creating a Connection

1. Navigate to the **Connections** page
2. Click **New Connection**
3. Select database type (PostgreSQL, MySQL, or SQLite)
4. Fill in connection details:
   - **Name**: Display name for the connection
   - **Host**: Database server hostname
   - **Port**: Database server port
   - **Database**: Database name
   - **Username**: Database username
   - **Password**: Database password
5. Click **Test Connection** to verify
6. Click **Save** to store the connection

### Executing Queries

1. Select a connection from the sidebar
2. Navigate to the **Query** page
3. Select the target database from the dropdown
4. Enter your SQL query in the editor
5. Click **Execute** or press `Ctrl+Enter`
6. View results in the table below

### Viewing Table Structure

1. Expand a connection in the sidebar
2. Click on a table name
3. View the **Columns**, **Indexes**, and **DDL** tabs

## Keyboard Shortcuts

| Shortcut     | Action                  |
| ------------ | ----------------------- |
| `Ctrl+Enter` | Execute query           |
| `Ctrl+S`     | Save query to bookmarks |
| `Ctrl+H`     | Toggle query history    |
| `Ctrl+L`     | Clear editor            |
| `Ctrl+D`     | Format SQL              |

## Project Structure

```
liteadmin/
├── crates/
│   ├── backend/           # Axum backend server
│   │   └── src/
│   │       ├── handlers/  # API route handlers
│   │       ├── db/        # Database operations
│   │       └── state.rs   # Application state
│   ├── frontend/          # Leptos WASM frontend
│   │   └── src/
│   │       ├── pages/     # Page components
│   │       ├── components/# Reusable components
│   │       ├── api/       # API client
│   │       └── state/     # Application state
│   └── shared/            # Shared types
└── target/                # Build output
```

## API Reference

### Connection Endpoints

| Method   | Endpoint                     | Description            |
| -------- | ---------------------------- | ---------------------- |
| `GET`    | `/api/connections`           | List all connections   |
| `POST`   | `/api/connections`           | Create new connection  |
| `GET`    | `/api/connections/{id}`      | Get connection details |
| `PUT`    | `/api/connections/{id}`      | Update connection      |
| `DELETE` | `/api/connections/{id}`      | Delete connection      |
| `POST`   | `/api/connections/{id}/test` | Test connection        |

### Schema Endpoints

| Method | Endpoint                                   | Description          |
| ------ | ------------------------------------------ | -------------------- |
| `GET`  | `/api/connections/{id}/schema`             | Get database schema  |
| `GET`  | `/api/connections/{id}/tables/{table}/def` | Get table definition |

### Query Endpoints

| Method | Endpoint                        | Description       |
| ------ | ------------------------------- | ----------------- |
| `POST` | `/api/connections/{id}/query`   | Execute SQL query |
| `GET`  | `/api/connections/{id}/history` | Get query history |

## Troubleshooting

### Common Issues

**1. "Function pg_get_tabledef does not exist"**

This indicates a PostgreSQL extension issue. The table DDL feature requires the `pg_get_tabledef` function from the `pg_utils` extension. For now, table indexes and column information are still available.

**2. "Connection refused" errors**

- Ensure the database server is running
- Verify firewall settings allow connections
- Check that the port number is correct

**3. "Database driver not found"**

Install the required database development libraries for your platform. See [Prerequisites](#prerequisites).

**4. Frontend not loading**

- Clear browser cache
- Ensure the backend is running on port 3000
- Check browser console for errors

### Logging

Enable debug logging by setting the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run -p sql-admin-backend
```

## Development

### Running Tests

```bash
# Run all tests
cargo test --all

# Run backend tests
cargo test -p sql-admin-backend

# Run frontend tests
cargo test -p sql-admin-frontend
```

### Code Formatting

```bash
# Format all code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check
```

### Linting

```bash
cargo clippy --all
```

## Contributing

Contributions are welcome! Please follow these steps:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/my-feature`
3. **Commit your changes**: `git commit -am 'Add new feature'`
4. **Push to the branch**: `git push origin feature/my-feature`
5. **Open a Pull Request**

### Coding Standards

- Follow Rust idioms and best practices
- Use `cargo fmt` for code formatting
- Run `cargo clippy` before submitting
- Write tests for new functionality
- Update documentation as needed

## Changelog

### v1.0.0 (2026-05-15)

- Added dark mode support
- Performance optimizations
- Database tools collection
- Query history feature
- Query bookmarks
- Data export functionality
- Initial release
- PostgreSQL, MySQL, SQLite support
- SQL query editor
- Table structure viewer

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

Built with:

- [Leptos](https://leptos.dev/) - The Rust WASM framework
- [Axum](https://github.com/tokio-rs/axum) - The Rust web framework
- [SQLx](https://github.com/launchbadge/sqlx) - The Rust SQL toolkit
- [TailwindCSS](https://tailwindcss.com/) - The utility-first CSS framework
