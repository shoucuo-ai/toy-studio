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

.sidebar-header h1 {
    font-size: 1.5rem;
    margin-bottom: 2rem;
    padding-bottom: 1rem;
    border-bottom: 1px solid #34495e;
}

.sidebar-menu {
    list-style: none;
    padding: 0;
}

.sidebar-menu li {
    margin-bottom: 0.5rem;
}

.sidebar-menu a {
    color: #ecf0f1;
    text-decoration: none;
    display: block;
    padding: 0.75rem 1rem;
    border-radius: 4px;
    transition: background-color 0.3s;
}

.sidebar-menu a:hover {
    background-color: #34495e;
}

.admin-content {
    flex: 1;
    padding: 2rem;
    background-color: #f5f6fa;
}

.content h2 {
    margin-bottom: 1rem;
    color: #2c3e50;
}
"#;
