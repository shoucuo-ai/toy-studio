pub mod layout;
pub mod dashboard_page;
pub mod routes;
pub mod app_store_page;
pub mod todo;
pub mod settings;
pub mod title_bar;
pub use settings::*;
pub const ADMIN_STYLES: &str = r#"
.admin-layout {
    display: flex;
    min-height: 100vh;
}

.admin-sidebar {
    width: 250px;
    background-color: #2c3e50;
    color: white;
    padding: 1rem;
}

.admin-content {
    flex: 1;
    padding: 2rem;
    background-color: #f5f6fa;
}

.sidebar-menu {
    list-style: none;
    padding: 0;
}

.sidebar-menu li {
    margin-bottom: 0.5rem;
}

.sidebar-menu a {
    color: white;
    text-decoration: none;
    display: block;
    padding: 0.5rem 1rem;
    border-radius: 4px;
}

.sidebar-menu a:hover {
    background-color: #34495e;
}

.dashboard-stats {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    margin-top: 2rem;
}

.stat-card {
    background: white;
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.form-group {
    margin-bottom: 1rem;
}

.form-group label {
    display: block;
    margin-bottom: 0.5rem;
}

.form-group input {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #ddd;
    border-radius: 4px;
}

button {
    background-color: #3498db;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
}

button:hover {
    background-color: #2980b9;
}
"#;
