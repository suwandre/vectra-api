#!/bin/bash
set -e

echo "🚀 Starting Vectra DEX API..."

# Build DATABASE_URL from RDS environment variables
if [ -n "$RDS_HOSTNAME" ] && [ -n "$RDS_DB_NAME" ] && [ -n "$RDS_USERNAME" ] && [ -n "$RDS_PASSWORD" ]; then
    RDS_PORT=${RDS_PORT:-5432}
    export DATABASE_URL="postgres://$RDS_USERNAME:$RDS_PASSWORD@$RDS_HOSTNAME:$RDS_PORT/$RDS_DB_NAME"
    echo "✅ DATABASE_URL constructed from RDS variables"
    echo "📊 Database: $RDS_DB_NAME at $RDS_HOSTNAME:$RDS_PORT"
    
    # Test database connection first
    echo "🔍 Testing database connection..."
    if timeout 30 psql "$DATABASE_URL" -c "SELECT 1;" >/dev/null 2>&1; then
        echo "✅ Database connection successful!"
        
        # Run database migrations with timeout
        echo "🔄 Running database migrations..."
        if timeout 60 sqlx migrate run --database-url "$DATABASE_URL"; then
            echo "✅ Migrations completed successfully!"
        else
            echo "⚠️ Migration failed or timed out, but continuing..."
        fi
    else
        echo "⚠️ Database connection failed, skipping migrations"
    fi
else
    echo "⚠️ RDS environment variables not found, skipping migrations"
fi

# Start the application
echo "🎯 Starting Vectra DEX API server..."
exec /app/vectra-api
