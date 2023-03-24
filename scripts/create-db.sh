#!/bin/bash

sqlite3 users.db < scripts/schema.sql
sqlite3 users.db < scripts/data.sql
