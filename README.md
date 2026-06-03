# LiteAdmin

A lightweight, modern database administration tool built entirely with Rust. LiteAdmin provides a web-based interface for managing database connections, executing SQL queries, and exploring database structures across multiple database types including SQL and NoSQL (Redb).

![preview](./dist/pic.gif)

## Features

- **Multi-Database Support**: Connect to PostgreSQL, MySQL, SQLite, and Redb databases
- **Connection Management**: Create, edit, test, and delete database connections with encrypted password storage
- **SQL Query Editor**: Execute SQL queries with syntax highlighting, table-aware query generation
- **Redb Key-Value Browser**: Browse Redb tables, search by key prefix, view key-value pairs with automatic type detection
- **Table Structure Viewer**: Browse table definitions, columns, indexes, and DDL
- **Query History**: Track and reuse past SQL queries
- **Query Bookmarks**: Save favorite queries for quick access
- **Dark Mode**: Full dark theme support
- **Database Tools**: Import/export data, SQL formatting, data comparison
- **Explorer Sidebar**: Database connections grouped by type with expandable schema tree
- **Responsive UI**: Modern, responsive interface built with TailwindCSS

## Tech Stack

| Layer        | Technology                           |
| ------------ | ------------------------------------ |
| Frontend     | Leptos 0.8 (Rust WASM), TailwindCSS |
| Backend      | Axum (Rust), SQLx, Redb             |
| Database     | PostgreSQL, MySQL, SQLite, Redb      |
| Architecture | DDD (Domain-Driven Design)          |
| Build Tool   | Cargo, Trunk                        |

## Prerequisites

- **Rust 1.95+** - [Install Rust](https://rustup.rs/)
- **trunk** - `cargo install trunk`
- **wasm32 target** - `rustup target add wasm32-unknown-unknown`
- **Node.js 26+** (optional, for TailwindCSS development)
- **cargo-watch** (optional, for backend hot reload) - `cargo install cargo-watch`
- **Database Drivers**:
  - PostgreSQL: `libpq` development libraries
  - MySQL: `mysql-client` development libraries
  - SQLite: Built-in support
  - Redb: Built-in support (pure Rust)

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

2. Build and run the backend:

```bash
cargo run -p sql-admin-interfaces
```

3. Build and run the frontend (requires wasm target):

```bash
rustup target add wasm32-unknown-unknown
cd crates/frontend
trunk serve --port 8080
```

### Development Mode

For active development with hot reload:

```bash
# Terminal 1: Start the backend (with hot reload)
.\scripts\start-backend.ps1

# Terminal 2: Start the frontend dev server (with hot reload)
.\scripts\start-frontend.ps1
```

Or manually:

```bash
# Terminal 1: Backend with cargo-watch
cargo watch -x 'run -p sql-admin-interfaces'

# Terminal 2: Frontend with trunk
cd crates/frontend
npm run build:css
trunk serve --port 8080
```

## Configuration

### Environment Variables

| Variable         | Description                             | Default                          |
| ---------------- | --------------------------------------- | -------------------------------- |
| `DATABASE_URL`   | SQLite database for storing connections | `sqlite:data/sql_admin.sqlite3?mode=rwc` |
| `SERVER_ADDR`    | Backend server address                  | `0.0.0.0:3000`                   |
| `RUST_LOG`       | Logging level                           | `sql_admin=debug,axum=info`      |
| `ENCRYPTION_KEY` | Key for encrypting connection passwords | (auto-generated in dev)          |

## Usage

### Creating a Connection

1. Navigate to the **Connections** page
2. Click **New Connection**
3. Select database type (PostgreSQL, MySQL, SQLite, or Redb)
4. Fill in connection details:
   - **Name**: Display name for the connection
   - **Host**: Database server hostname
   - **Port**: Database server port
   - **Database**: Database name (or file path for SQLite/Redb)
   - **Username**: Database username
   - **Password**: Database password (encrypted at rest)
5. Click **Test Connection** to verify
6. Click **Save** to store the connection

### Executing SQL Queries

1. Navigate to the **SQL Query** page
2. Select a connection from the dropdown (SQL databases only)
3. Select a table from the table dropdown to auto-generate a SELECT statement
4. Enter your SQL query in the editor
5. Click **Execute** or press `Ctrl+Enter`
6. View results in the table below

### Browsing Redb Databases

1. Expand the Redb group in the Explorer sidebar
2. Click a connection to view its tables
3. Click a table to browse its key-value pairs
4. Use the Key Prefix search to filter entries
5. Navigate with pagination controls

### Viewing Table Structure

1. Expand a connection in the Explorer sidebar
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
sql-admin/
├── crates/
│   ├── api-types/         # Shared request/response DTOs
│   ├── domain/            # Domain layer (aggregates, repository traits)
│   ├── infrastructure/    # Infrastructure layer (SQLite, Redb, encryption, pool)
│   ├── application/       # Application layer (handlers, use cases)
│   ├── interfaces/        # Interface layer (Axum server, routes, middleware)
│   ├── frontend/          # Leptos WASM frontend
│   │   └── src/
│   │       ├── pages/     # Page components
│   │       ├── components/# Reusable components
│   │       ├── api/       # API client
│   │       └── state/     # Application state
│   └── shared/            # Shared utilities
├── scripts/               # Development scripts
│   ├── start-backend.ps1  # Backend dev server with hot reload
│   └── start-frontend.ps1 # Frontend dev server with hot reload
└── data/                  # Runtime data directory
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

### Redb Endpoints

| Method | Endpoint                                      | Description              |
| ------ | --------------------------------------------- | ------------------------ |
| `GET`  | `/api/connections/{id}/redb/tables`           | List Redb tables         |
| `POST` | `/api/connections/{id}/redb/query`            | Query Redb key-value pairs |

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

**5. Redb table returns "unsupported types"**

Redb tables use strongly-typed key/value pairs. If you encounter this error, check the backend logs for the detected type names and report them so we can add support.

### Logging

Enable debug logging by setting the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run -p sql-admin-interfaces
```

## Development

### Running Tests

```bash
# Run all tests
cargo test --all
```

### Code Formatting

```bash
cargo fmt --all
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

### v1.1.0 (2026-06-03)

- Added Redb (key-value database) support with table browser and key prefix search
- Added database type grouping in Explorer sidebar
- Added table dropdown in SQL Query page for auto-generating SELECT statements
- Added database type badges in Connections page
- Added development scripts with hot reload support
- Fixed SQLite query results returning null values for untyped columns
- Fixed Redb table type detection with automatic type matching
- Improved error reporting with actual error messages instead of generic 500 responses

### v1.0.0 (2026-05-15)

- Initial release
- PostgreSQL, MySQL, SQLite support
- SQL query editor
- Table structure viewer
- Query history and bookmarks
- Dark mode support
- Data export functionality
- Database tools collection

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

Built with:

- [Leptos](https://leptos.dev/) - The Rust WASM framework
- [Axum](https://github.com/tokio-rs/axum) - The Rust web framework
- [SQLx](https://github.com/launchbadge/sqlx) - The Rust SQL toolkit
- [Redb](https://github.com/cberner/redb) - Simple ACID key-value database
- [TailwindCSS](https://tailwindcss.com/) - The utility-first CSS framework
