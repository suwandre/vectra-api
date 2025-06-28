#!/bin/bash
set -e

echo "🚀 Starting Vectra DEX API..."

# Build DATABASE_URL from RDS environment variables
if [ -n "$RDS_HOSTNAME" ] && [ -n "$RDS_DB_NAME" ] && [ -n "$RDS_USERNAME" ] && [ -n "$RDS_PASSWORD" ]; then
    RDS_PORT=${RDS_PORT:-5432}
    export DATABASE_URL="postgres://$RDS_USERNAME:$RDS_PASSWORD@$RDS_HOSTNAME:$RDS_PORT/$RDS_DB_NAME"
    echo "✅ DATABASE_URL constructed from RDS variables"
    echo "📊 Database: $RDS_DB_NAME at $RDS_HOSTNAME:$RDS_PORT"
else
    echo "⚠️  RDS environment variables not found"
fi

# Wait for database to be ready (if DATABASE_URL is set)
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
