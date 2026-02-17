#!/bin/bash
set -e

# Run all migrations as the PostgreSQL superuser.
# SET ROLE lines are commented out so cross-schema REFERENCES
# (e.g. front_office.patients -> auth.users) don't fail due to
# missing REFERENCES grants. This is fine for dev; production
# would use a privileged migration runner.

for f in /migrations/*.sql; do
    echo "Running migration: $f"
    sed 's/^SET ROLE/-- &/' "$f" \
        | psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB"
done
