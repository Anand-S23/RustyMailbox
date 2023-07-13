import { Pool } from 'pg';

export const pg_pool = new Pool({
    user: process.env.POSTGRES_USER || 'postgres',
    password: process.env.POSTGRES_PASSWORD || 'password123',
    host: process.env.POSTGRES_HOST || 'localhost',
    port: 5432,
    database: process.env.POSTGRES_DB || 'mailbox_db'
});

export const query = async (query: string, params: any[]) => {
    const {rows, fields} = await pg_pool.query(query, params)
        .catch((err) => { throw err });
    return rows;
}
