mod ast;
mod parser;

#[cfg(test)]
mod tests;

use itertools::Itertools;

pub use self::ast::{Alias, Relation, Method, RelVariance};

#[derive(Debug)]
pub enum Error {
    ParseError,
    InvalidAliasMethod,
    DuplicateResources,
    DuplicateRelations,
}

#[derive(Debug)]
pub struct Routes {
    pub resources: Vec<Resource>,
}

impl Routes {
    pub fn new(dsl: &str) -> Result<Routes, Error> {
        let routes = ast::Routes::parse(dsl).map_err(|_| Error::ParseError)?;
        Routes::check(routes)
    }

    fn check(routes: ast::Routes) -> Result<Routes, Error> {
        routes.resources.into_iter().map(Resource::check).collect::<Result<Vec<_>, _>>().and_then(|resources| {
            duplicate_resources(&resources)?;
            Ok(Routes {
                resources: resources,
            })
        })
    }
}

#[derive(Debug)]
pub struct Resource {
    pub resource: String,
    pub relations: Vec<Relation>,
    pub aliases: Vec<Alias>,
}

impl Resource {
    fn check(resource: ast::Resource) -> Result<Resource, Error> {
        let mut relations = vec![];
        let mut aliases = vec![];

        for attr in resource.attrs {
            match attr {
                ast::ResourceAttr::Relation(rel)    => relations.push(rel),
                ast::ResourceAttr::Alias(alias)     => aliases.push(check_alias(alias)?),
            }
        }

        duplicate_rels(&relations)?;

        Ok(Resource {
            resource: resource.resource,
            relations: relations,
            aliases: aliases,
        })
    }
}

fn check_alias(alias: Alias) -> Result<Alias, Error> {
    match alias.method {
        Method::Get | Method::Index => Ok(alias),
        _                           => Err(Error::InvalidAliasMethod),
    }
}

fn duplicate_resources(resources: &[Resource]) -> Result<(), Error> {
    if resources.len() == resources.iter().map(|resource| &resource.resource).unique().count() {
        Ok(())
    } else {
        Err(Error::DuplicateResources)
    }
}

fn duplicate_rels(rels: &[Relation]) -> Result<(), Error> {
    if rels.len() == rels.iter().map(|rel| &rel.relation).unique().count() {
        Ok(())
    } else {
        Err(Error::DuplicateRelations)
    }
}
