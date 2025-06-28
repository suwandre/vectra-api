#!/bin/bash
set -e

echo "🚀 Starting Vectra DEX API..."

# Build DATABASE_URL from RDS environment variables with SSL support
if [ -n "$RDS_HOSTNAME" ] && [ -n "$RDS_DB_NAME" ] && [ -n "$RDS_USERNAME" ] && [ -n "$RDS_PASSWORD" ]; then
    RDS_PORT=${RDS_PORT:-5432}
    
    # Add SSL parameters for RDS connection
    export DATABASE_URL="postgres://$RDS_USERNAME:$RDS_PASSWORD@$RDS_HOSTNAME:$RDS_PORT/$RDS_DB_NAME?sslmode=require"
    echo "✅ DATABASE_URL constructed from RDS variables with SSL"
    echo "📊 Database: $RDS_DB_NAME at $RDS_HOSTNAME:$RDS_PORT"
    
    # Test database connection with SSL
    echo "🔍 Testing database connection..."
    if timeout 30 psql "$DATABASE_URL" -c "SELECT 1;" >/dev/null 2>&1; then
        echo "✅ Database connection successful!"
        
        echo "🔄 Running database migrations..."
        if timeout 60 sqlx migrate run --database-url "$DATABASE_URL"; then
            echo "✅ Migrations completed successfully!"
        else
            echo "⚠️ Migration failed, but continuing..."
        fi
    else
        echo "⚠️ Database connection failed, skipping migrations"
        echo "🔍 Debug: Trying connection without SSL verification..."
        # Try with sslmode=disable for debugging
        DEBUG_URL="postgres://$RDS_USERNAME:$RDS_PASSWORD@$RDS_HOSTNAME:$RDS_PORT/$RDS_DB_NAME?sslmode=disable"
        if timeout 10 psql "$DEBUG_URL" -c "SELECT 1;" >/dev/null 2>&1; then
            echo "✅ Connection works without SSL - RDS requires SSL configuration"
        else
            echo "❌ Connection failed even without SSL - check credentials/network"
        fi
    fi
else
    echo "⚠️ RDS environment variables not found"
fi

echo "🎯 Starting Vectra DEX API server..."
exec /app/vectra-api
