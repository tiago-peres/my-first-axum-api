// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod users
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug, Clone, PartialEq,)] pub struct GetAllUsers
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
} }}}