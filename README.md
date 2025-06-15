## Wordford 🪼🦀

Everything you need to run your own content management system with ease. Wordford is designed for simplicity and speed, empowering you to deliver high-quality content to your users effortlessly and efficiently.

## Getting Started

You will need to configure your `.env` file in order to run Wordford. The environment file should have the following structure:

```env
DATABASE_URL=sqlite://wordford.db
```

## Deploying to a Server

Deploying to a server is a breeze. We've included a script that will build
and run Wordford in a Docker container and start it running on port `8088`.

Simply run the following command on the server that you wish to run Wordford on:

```shell
./deploy.sh
```
