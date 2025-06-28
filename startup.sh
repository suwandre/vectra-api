#!/bin/bash
set -e

echo "🚀 Starting Vectra DEX API..."

# Debug: Print all RDS variables
echo "🔍 Debug - RDS Environment Variables:"
echo "RDS_HOSTNAME: $RDS_HOSTNAME"
echo "RDS_USERNAME: $RDS_USERNAME"
echo "RDS_DB_NAME: $RDS_DB_NAME"
echo "RDS_PORT: $RDS_PORT"
echo "RDS_PASSWORD: [REDACTED]"

# Build DATABASE_URL from RDS environment variables with SSL support
if [ -n "$RDS_HOSTNAME" ] && [ -n "$RDS_DB_NAME" ] && [ -n "$RDS_USERNAME" ] && [ -n "$RDS_PASSWORD" ]; then
    RDS_PORT=${RDS_PORT:-5432}
    
    export DATABASE_URL="postgres://$RDS_USERNAME:$RDS_PASSWORD@$RDS_HOSTNAME:$RDS_PORT/$RDS_DB_NAME?sslmode=require"
    echo "✅ DATABASE_URL constructed from RDS variables with SSL"
    echo "📊 Database: $RDS_DB_NAME at $RDS_HOSTNAME:$RDS_PORT"
    
    # Debug: Print constructed URL (without password)
    echo "🔍 Debug - DATABASE_URL format: postgres://$RDS_USERNAME:[REDACTED]@$RDS_HOSTNAME:$RDS_PORT/$RDS_DB_NAME?sslmode=require"
    
    # Test database connection with more verbose output
    echo "🔍 Testing database connection..."
    if timeout 30 psql "$DATABASE_URL" -c "SELECT 1;" 2>&1; then
        echo "✅ Database connection successful!"
        
        echo "🔄 Running database migrations..."
        if timeout 60 sqlx migrate run --database-url "$DATABASE_URL" 2>&1; then
            echo "✅ Migrations completed successfully!"
        else
            echo "⚠️ Migration failed, but continuing..."
        fi
    else
        echo "⚠️ Database connection failed"
        echo "🔍 Attempting connection with verbose psql output..."
        psql "$DATABASE_URL" -c "SELECT 1;" -v ON_ERROR_STOP=1 || true
    fi
else
    echo "⚠️ RDS environment variables not found"
fi

echo "🎯 Starting Vectra DEX API server..."
exec /app/vectra-api
