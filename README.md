# user-management-rust

API using `actix-web` with authentication using JWT

For the construction of this API I follow the tutorial in this link

```
https://auth0.com/blog/build-an-api-in-rust-with-jwt-authentication-using-actix-web/
```

until some point because the implementation of the auth using JWT was using `jsonwebtoken` library.

There is one error in the tutorial, in order to run the schema generation with:

```
diesel print-schema > src/schema.rs
```

we must run the migration first

```
diesel migration run
```

To sign the token I used `RS256` algorithm. To generate the private and public key I used `openssl`. You can generate your own keys using the follow command.

```
openssl genrsa -out private-key.pem 2048
openssl rsa -in private-key.pem -outform PEM -pubout -out public-key.pem
```

I use the private and public keys as environment variables, for this you need to remove the new lines and replace it with `\n` character. Another possibility to get the private and public keys is using a file with the certificates and the code to load the keys would be like this

```
let key_private_key = EncodingKey::from_rsa_pem(include_bytes!("../cert/private-key.pem")).unwrap();
let key_public_key = DecodingKey::from_rsa_pem(include_bytes!("../cert/public-key.pem")).unwrap()
```
