import express, { Express, Request, Response } from 'express';
import dotenv from 'dotenv';
import cors from 'cors';

// Initalize App //

const app: Express = express();
app.use(express.json());
app.use(cors<Request>());

dotenv.config();

// Routes //

app.get('/', (req: Request, res: Response) => {
    res.send("Hello World");
})

// Start Server //

const port = process.env.PORT || 8080;
app.listen(port, () => {
    console.log(`⚡️[server]: Server running on port ${port}`);
});
