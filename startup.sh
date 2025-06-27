#!/bin/bash
set -e

echo "🚀 Starting Vectra DEX API..."

# Wait for database to be ready (if using external database)
if [ -n "$DATABASE_URL" ]; then
    echo "📊 Waiting for database to be ready..."
    
    # Extract database connection details for health check
    DB_HOST=$(echo $DATABASE_URL | sed -n 's/.*@\([^:]*\):.*/\1/p')
    DB_PORT=$(echo $DATABASE_URL | sed -n 's/.*:\([0-9]*\)\/.*/\1/p')
    
    # Wait for database connection
    timeout 60 bash -c "until nc -z $DB_HOST $DB_PORT; do sleep 1; done"
    echo "✅ Database is ready!"
    
    # Run database migrations
    echo "🔄 Running database migrations..."
    sqlx migrate run --database-url "$DATABASE_URL"
    echo "✅ Migrations completed!"
else
    echo "⚠️  No DATABASE_URL provided, skipping migrations"
fi

# Start the application
echo "🎯 Starting Vectra DEX API server..."
exec /app/vectra-api
