// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod users
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct UpdateUserParams<T1: cornucopia_async::StringSql,> { pub name: T1,pub id: i32,}#[derive( Debug, Clone, PartialEq,)] pub struct GetAllUsers
{ pub id : i32,pub name : String,}pub struct GetAllUsersBorrowed<'a> { pub id : i32,pub name : &'a str,}
impl<'a> From<GetAllUsersBorrowed<'a>> for GetAllUsers
{
    fn from(GetAllUsersBorrowed { id,name,}: GetAllUsersBorrowed<'a>) ->
    Self { Self { id,name: name.into(),} }
}pub struct GetAllUsersQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> GetAllUsersBorrowed,
    mapper: fn(GetAllUsersBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> GetAllUsersQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(GetAllUsersBorrowed) -> R) ->
    GetAllUsersQuery<'a,C,R,N>
    {
        GetAllUsersQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct InsertUser
{ pub id : i32,pub name : String,}pub struct InsertUserBorrowed<'a> { pub id : i32,pub name : &'a str,}
impl<'a> From<InsertUserBorrowed<'a>> for InsertUser
{
    fn from(InsertUserBorrowed { id,name,}: InsertUserBorrowed<'a>) ->
    Self { Self { id,name: name.into(),} }
}pub struct InsertUserQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> InsertUserBorrowed,
    mapper: fn(InsertUserBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> InsertUserQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(InsertUserBorrowed) -> R) ->
    InsertUserQuery<'a,C,R,N>
    {
        InsertUserQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct GetUser
{ pub id : i32,pub name : String,}pub struct GetUserBorrowed<'a> { pub id : i32,pub name : &'a str,}
impl<'a> From<GetUserBorrowed<'a>> for GetUser
{
    fn from(GetUserBorrowed { id,name,}: GetUserBorrowed<'a>) ->
    Self { Self { id,name: name.into(),} }
}pub struct GetUserQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> GetUserBorrowed,
    mapper: fn(GetUserBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> GetUserQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(GetUserBorrowed) -> R) ->
    GetUserQuery<'a,C,R,N>
    {
        GetUserQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct DeleteUser
{ pub id : i32,pub name : String,}pub struct DeleteUserBorrowed<'a> { pub id : i32,pub name : &'a str,}
impl<'a> From<DeleteUserBorrowed<'a>> for DeleteUser
{
    fn from(DeleteUserBorrowed { id,name,}: DeleteUserBorrowed<'a>) ->
    Self { Self { id,name: name.into(),} }
}pub struct DeleteUserQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> DeleteUserBorrowed,
    mapper: fn(DeleteUserBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> DeleteUserQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(DeleteUserBorrowed) -> R) ->
    DeleteUserQuery<'a,C,R,N>
    {
        DeleteUserQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct UpdateUser
{ pub id : i32,pub name : String,}pub struct UpdateUserBorrowed<'a> { pub id : i32,pub name : &'a str,}
impl<'a> From<UpdateUserBorrowed<'a>> for UpdateUser
{
    fn from(UpdateUserBorrowed { id,name,}: UpdateUserBorrowed<'a>) ->
    Self { Self { id,name: name.into(),} }
}pub struct UpdateUserQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> UpdateUserBorrowed,
    mapper: fn(UpdateUserBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> UpdateUserQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(UpdateUserBorrowed) -> R) ->
    UpdateUserQuery<'a,C,R,N>
    {
        UpdateUserQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn get_all_users() -> GetAllUsersStmt
{ GetAllUsersStmt(cornucopia_async::private::Stmt::new("SELECT id, name FROM users")) } pub struct
GetAllUsersStmt(cornucopia_async::private::Stmt); impl GetAllUsersStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> GetAllUsersQuery<'a,C,
GetAllUsers, 0>
{
    GetAllUsersQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { GetAllUsersBorrowed { id: row.get(0),name: row.get(1),} }, mapper: |it| { <GetAllUsers>::from(it) },
    }
} }pub fn insert_user() -> InsertUserStmt
{ InsertUserStmt(cornucopia_async::private::Stmt::new("INSERT INTO users (name) VALUES ($1) RETURNING id, name")) } pub struct
InsertUserStmt(cornucopia_async::private::Stmt); impl InsertUserStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
name: &'a T1,) -> InsertUserQuery<'a,C,
InsertUser, 1>
{
    InsertUserQuery
    {
        client, params: [name,], stmt: &mut self.0, extractor:
        |row| { InsertUserBorrowed { id: row.get(0),name: row.get(1),} }, mapper: |it| { <InsertUser>::from(it) },
    }
} }pub fn get_user() -> GetUserStmt
{ GetUserStmt(cornucopia_async::private::Stmt::new("SELECT id, name FROM users WHERE id = $1")) } pub struct
GetUserStmt(cornucopia_async::private::Stmt); impl GetUserStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
id: &'a i32,) -> GetUserQuery<'a,C,
GetUser, 1>
{
    GetUserQuery
    {
        client, params: [id,], stmt: &mut self.0, extractor:
        |row| { GetUserBorrowed { id: row.get(0),name: row.get(1),} }, mapper: |it| { <GetUser>::from(it) },
    }
} }pub fn delete_user() -> DeleteUserStmt
{ DeleteUserStmt(cornucopia_async::private::Stmt::new("DELETE FROM users WHERE id = $1 RETURNING id, name")) } pub struct
DeleteUserStmt(cornucopia_async::private::Stmt); impl DeleteUserStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
id: &'a i32,) -> DeleteUserQuery<'a,C,
DeleteUser, 1>
{
    DeleteUserQuery
    {
        client, params: [id,], stmt: &mut self.0, extractor:
        |row| { DeleteUserBorrowed { id: row.get(0),name: row.get(1),} }, mapper: |it| { <DeleteUser>::from(it) },
    }
} }pub fn update_user() -> UpdateUserStmt
{ UpdateUserStmt(cornucopia_async::private::Stmt::new("UPDATE users SET name = $1 WHERE id = $2 RETURNING id, name")) } pub struct
UpdateUserStmt(cornucopia_async::private::Stmt); impl UpdateUserStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
name: &'a T1,id: &'a i32,) -> UpdateUserQuery<'a,C,
UpdateUser, 2>
{
    UpdateUserQuery
    {
        client, params: [name,id,], stmt: &mut self.0, extractor:
        |row| { UpdateUserBorrowed { id: row.get(0),name: row.get(1),} }, mapper: |it| { <UpdateUser>::from(it) },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,> cornucopia_async::Params<'a,
UpdateUserParams<T1,>, UpdateUserQuery<'a, C,
UpdateUser, 2>, C> for UpdateUserStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    UpdateUserParams<T1,>) -> UpdateUserQuery<'a, C,
    UpdateUser, 2>
    { self.bind(client, &params.name,&params.id,) }
}}}