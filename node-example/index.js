const express = require('express');
const { Pool } = require('pg');

const pool = new Pool({
    user: 'mzz',
    host: 'localhost',
    database: 'mzz',
    password: '',
    port: 5432,
});

const app = express();
app.use(express.json());

app.get('/pois', async (req, res) => {
    const page = req.query.page || 0;
    const perPage = req.query.per_page || 10;
    const offset = page * perPage;

    const start = performance.now();
    const result = await pool.query(
        'SELECT * FROM pois LIMIT $1 OFFSET $2',
        [perPage, offset]
    );
    const end = performance.now();

    res.json({
        data: result.rows,
        queryTime: `${end - start} ms`,
    });
});

app.get('/pois-by-radius', async (req, res) => {
    const { latitude, longitude, radius } = req.query;

    const start = performance.now();
    const result = await pool.query(
        `
      SELECT *
      FROM pois
      WHERE ST_DWithin(ST_MakePoint($1, $2)::geography, ST_MakePoint(pois.latitude, pois.longitude)::geography, $3)
    `,
        [latitude, longitude, radius]
    );
    const end = performance.now();

    res.json({
        data: result.rows,
        queryTime: `${end - start} ms`,
    });
});

app.listen(3000, () => {
    console.log('Server is running on port 3000');
});