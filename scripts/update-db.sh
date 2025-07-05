#!/bin/bash
set -e

echo "🔄 Updating database schema and query cache..."

# Apply any new migrations
echo "📊 Running database migrations..."
sqlx migrate run

# Update query cache
echo "🗂️  Updating SQLx query cache..."
cargo sqlx prepare --workspace

# Stage the updated cache
echo "📝 Staging query cache for commit..."
git add .sqlx/

echo "✅ Database and query cache updated!"
echo "💡 Don't forget to commit the .sqlx/ changes!"
