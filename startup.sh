#!/bin/bash
set -e

echo "🚀 Starting Vectra DEX API..."

# Build DATABASE_URL from RDS environment variables with proper escaping
if [ -n "$RDS_HOSTNAME" ] && [ -n "$RDS_DB_NAME" ] && [ -n "$RDS_USERNAME" ] && [ -n "$RDS_PASSWORD" ]; then
    RDS_PORT=${RDS_PORT:-5432}
    
    # URL-encode the password to handle special characters
    ENCODED_PASSWORD=$(printf '%s\n' "$RDS_PASSWORD" | sed 's/[[\.*^$()+?{|]/\\&/g')
    
    export DATABASE_URL="postgres://$RDS_USERNAME:$ENCODED_PASSWORD@$RDS_HOSTNAME:$RDS_PORT/$RDS_DB_NAME"
    echo "✅ DATABASE_URL constructed from RDS variables"
    echo "📊 Database: $RDS_DB_NAME at $RDS_HOSTNAME:$RDS_PORT"
    
    # Test database connection
    echo "🔍 Testing database connection..."
    if timeout 30 psql "$DATABASE_URL" -c "SELECT 1;" >/dev/null 2>&1; then
        echo "✅ Database connection successful!"
        
        echo "🔄 Running database migrations..."
        if timeout 60 sqlx migrate run --database-url "$DATABASE_URL"; then
            echo "✅ Migrations completed successfully!"
        else
            echo "⚠️ Migration failed or timed out, but continuing..."
        fi
    else
        echo "⚠️ Database connection failed, skipping migrations"
        echo "🔍 Debugging connection..."
        echo "Host: $RDS_HOSTNAME"
        echo "Port: $RDS_PORT"
        echo "Database: $RDS_DB_NAME"
        echo "Username: $RDS_USERNAME"
    fi
else
    echo "⚠️ RDS environment variables not found"
fi

# Start the application
echo "🎯 Starting Vectra DEX API server..."
exec /app/vectra-api
