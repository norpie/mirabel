pub(crate) mod surrealdb;

#[cfg(test)]
pub mod tests {
    use backend_derive::{named_struct};
    use serde::{Deserialize, Serialize};
    use surrealdb::sql::Thing;
    use futures::StreamExt;
    use log::{info, debug};

    use crate::repository::traits::{
        Entity, Repository, FieldSearchableRepository, FieldFindableRepository,
        PublicEntityRepository, AssociatedEntityRepository, ThroughputRepository,
        LiveRepository, FieldFindableStruct, FieldSearchableStruct
    };
    use crate::dto::page::{PageRequest, PageResponse};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[named_struct()]
    pub struct TestEntity {
        id: Option<Thing>,
        name: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[named_struct()]
    pub struct TestEntityChild {
        id: Option<Thing>,
        name: String,
    }

    impl Entity for TestEntity {
        type ID = String;

        fn id(&self) -> Option<Self::ID> {
            self.id.as_ref().map(|t| t.id.to_string())
        }
    }

    impl Entity for TestEntityChild {
        type ID = String;

        fn id(&self) -> Option<Self::ID> {
            self.id.as_ref().map(|t| t.id.to_string())
        }
    }

    impl FieldFindableStruct for TestEntity {
        fn filterable_fields() -> &'static [&'static str] {
            &["name"]
        }
    }

    impl FieldSearchableStruct for TestEntity {
        fn searchable_fields() -> &'static [&'static str] {
            &["name"]
        }
    }

    impl FieldFindableStruct for TestEntityChild {
        fn filterable_fields() -> &'static [&'static str] {
            &["name"]
        }
    }

    impl FieldSearchableStruct for TestEntityChild {
        fn searchable_fields() -> &'static [&'static str] {
            &["name"]
        }
    }

    pub async fn test_repository(repo: impl Repository<TestEntity>) {
        info!("Starting test_repository");
        // Create a test entity
        let entity = TestEntity {
            id: None,
            name: "Test Entity".to_string(),
        };
        debug!("Created test entity: {:?}", entity);

        // Test save
        let saved_entity = repo.save(entity.clone()).await.expect("Failed to save entity");
        info!("Saved entity with ID: {:?}", saved_entity.id());
        assert!(saved_entity.id().is_some(), "Saved entity should have an ID");

        let id = saved_entity.id().unwrap();
        debug!("Using ID for further operations: {}", id);

        // Test find
        let found_entity = repo.find(&id).await.expect("Failed to find entity");
        debug!("Found entity: {:?}", found_entity);
        assert!(found_entity.is_some(), "Entity should be found");
        let found_entity = found_entity.unwrap();
        assert_eq!(found_entity.name, saved_entity.name, "Found entity should match saved entity");
        info!("Entity found successfully");

        // Test exists
        let exists = repo.exists(&id).await.expect("Failed to check existence");
        debug!("Entity exists check: {}", exists);
        assert!(exists, "Entity should exist");

        // Test count
        let count = repo.count().await.expect("Failed to count entities");
        info!("Entity count: {}", count);
        assert!(count > 0, "Count should be greater than 0");

        // Test delete
        debug!("Attempting to delete entity with ID: {}", id);
        repo.delete(&id).await.expect("Failed to delete entity");
        let exists_after_delete = repo.exists(&id).await.expect("Failed to check existence");
        debug!("Entity exists after deletion: {}", exists_after_delete);
        assert!(!exists_after_delete, "Entity should not exist after deletion");
        info!("test_repository completed successfully");
    }

    pub async fn test_field_searchable_repository(repo: impl FieldSearchableRepository<TestEntity>) {
        info!("Starting test_field_searchable_repository");
        // Create test entities
        let entity1 = TestEntity {
            id: None,
            name: "Unique Test Entity".to_string(),
        };
        let entity2 = TestEntity {
            id: None,
            name: "Another Test Entity".to_string(),
        };
        debug!("Created test entities: {:?}, {:?}", entity1, entity2);

        let saved1 = repo.save(entity1).await.expect("Failed to save entity 1");
        let saved2 = repo.save(entity2).await.expect("Failed to save entity 2");
        info!("Saved entities with IDs: {:?}, {:?}", saved1.id(), saved2.id());

        // Test search
        let page_req = PageRequest::new(1, 10); // Changed from 0 to 1
        debug!("Searching for 'Unique' with page request: {:?}", page_req);
        let search_results = repo.search(&["name"], "Unique", page_req).await.expect("Failed to search");
        info!("Search returned {} results", search_results.data().len());

        assert_eq!(search_results.data().len(), 1, "Search should return one result");
        assert_eq!(search_results.data()[0].name, saved1.name, "Search result should match");
        debug!("Search result matched as expected");

        // Clean up
        debug!("Cleaning up test entities");
        if let Some(id) = saved1.id() {
            repo.delete(&id).await.expect("Failed to delete entity 1");
            debug!("Deleted entity 1 with ID: {}", id);
        }
        if let Some(id) = saved2.id() {
            repo.delete(&id).await.expect("Failed to delete entity 2");
            debug!("Deleted entity 2 with ID: {}", id);
        }
        info!("test_field_searchable_repository completed successfully");
    }

    pub async fn test_field_findable_repository(repo: impl FieldFindableRepository<TestEntity>) {
        info!("Starting test_field_findable_repository");
        // Create test entities
        let entity1 = TestEntity {
            id: None,
            name: "FindableEntity".to_string(),
        };
        let entity2 = TestEntity {
            id: None,
            name: "OtherEntity".to_string(),
        };
        debug!("Created test entities: {:?}, {:?}", entity1, entity2);

        let saved1 = repo.save(entity1).await.expect("Failed to save entity 1");
        let saved2 = repo.save(entity2).await.expect("Failed to save entity 2");
        info!("Saved entities with IDs: {:?}, {:?}", saved1.id(), saved2.id());

        // Test find by field
        let page_req = PageRequest::new(1, 10); // Changed from 0 to 1
        debug!("Finding by field 'name'='FindableEntity' with page request: {:?}", page_req);
        let find_results = repo.find_by_field("name", "FindableEntity", page_req).await
            .expect("Failed to find by field");
        info!("Find by field returned {} results", find_results.data().len());

        assert_eq!(find_results.data().len(), 1, "Find by field should return one result");
        assert_eq!(find_results.data()[0].name, saved1.name, "Found entity should match");
        debug!("Find by field result matched as expected");

        // Test exists by field
        debug!("Checking if entity exists by field 'name'='FindableEntity'");
        let exists = repo.exists_by_field("name", "FindableEntity").await
            .expect("Failed to check existence by field");
        debug!("Entity exists check: {}", exists);
        assert!(exists, "Entity should exist by field");

        debug!("Checking if entity exists by field 'name'='NonexistentEntity'");
        let nonexistent = repo.exists_by_field("name", "NonexistentEntity").await
            .expect("Failed to check existence by field");
        debug!("Nonexistent entity exists check: {}", nonexistent);
        assert!(!nonexistent, "Nonexistent entity should not exist");

        // Clean up
        debug!("Cleaning up test entities");
        if let Some(id) = saved1.id() {
            repo.delete(&id).await.expect("Failed to delete entity 1");
            debug!("Deleted entity 1 with ID: {}", id);
        }
        if let Some(id) = saved2.id() {
            repo.delete(&id).await.expect("Failed to delete entity 2");
            debug!("Deleted entity 2 with ID: {}", id);
        }
        info!("test_field_findable_repository completed successfully");
    }

    pub async fn test_public_entity_repository(repo: impl PublicEntityRepository<TestEntity>) {
        info!("Starting test_public_entity_repository");
        // Create multiple test entities
        let entities = vec![
            TestEntity { id: None, name: "Public Entity 1".to_string() },
            TestEntity { id: None, name: "Public Entity 2".to_string() },
            TestEntity { id: None, name: "Public Entity 3".to_string() },
        ];
        debug!("Created {} test entities", entities.len());

        let mut saved_ids = Vec::new();
        for entity in entities {
            let saved = repo.save(entity).await.expect("Failed to save entity");
            if let Some(id) = saved.id() {
                debug!("Saved entity with ID: {}", id);
                saved_ids.push(id);
            }
        }
        info!("Saved {} entities", saved_ids.len());

        // Test find_all
        let page_req = PageRequest::new(1, 10); // Changed from 0 to 1
        debug!("Finding all entities with page request: {:?}", page_req);
        let all_entities = repo.find_all(page_req).await.expect("Failed to find all entities");
        info!("Found {} entities", all_entities.data().len());

        assert!(all_entities.data().len() >= 3, "Should find at least 3 entities");

        // Test pagination
        let page_req = PageRequest::new(1, 2); // Changed from 0 to 1
        debug!("Getting first page with size 2");
        let first_page = repo.find_all(page_req).await.expect("Failed to get first page");
        info!("First page contains {} items", first_page.data().len());
        assert_eq!(first_page.data().len(), 2, "First page should have 2 items");

        let page_req = PageRequest::new(2, 2); // Changed from 1 to 2
        debug!("Getting second page with size 2");
        let second_page = repo.find_all(page_req).await.expect("Failed to get second page");
        info!("Second page contains {} items", second_page.data().len());
        assert!(!second_page.data().is_empty(), "Second page should have at least 1 item");

        // Clean up
        debug!("Cleaning up {} test entities", saved_ids.len());
        for id in saved_ids {
            repo.delete(&id).await.expect("Failed to delete entity");
            debug!("Deleted entity with ID: {}", id);
        }
        info!("test_public_entity_repository completed successfully");
    }

    pub async fn test_associated_entity_one_to_one(
        entity_repo: impl AssociatedEntityRepository<TestEntity, TestEntityChild>,
        child_repo: impl Repository<TestEntityChild>
    ) {
        info!("Starting test_associated_entity_repository");
        // Create parent and child entities
        let parent = TestEntity { id: None, name: "Parent Entity".to_string() };
        debug!("Created parent entity: {:?}", parent);
        let saved_parent = entity_repo.save(parent).await.expect("Failed to save parent");
        let parent_id = saved_parent.id().expect("Parent should have ID");
        info!("Saved parent entity with ID: {}", parent_id);

        let child = TestEntityChild { id: None, name: "Child Entity".to_string() };
        debug!("Created child entity: {:?}", child);
        let saved_child = child_repo.save(child).await.expect("Failed to save child");
        let child_id = saved_child.id().expect("Child should have ID");
        info!("Saved child entity with ID: {}", child_id);

        // Test one-to-one relationship methods
        debug!("Relating parent {} to child {}", parent_id, child_id);
        entity_repo.relate(&child_id, &parent_id).await.expect("Failed to relate entities");

        let related = entity_repo.find_related(&parent_id).await.expect("Failed to find related");
        debug!("Found related entity: {:?}", related);
        assert!(related.is_some(), "Related entity should exist");

        let exists_related = entity_repo.exists_related(&parent_id).await.expect("Failed to check existence");
        debug!("Related entity exists: {}", exists_related);
        assert!(exists_related, "Related entity should exist");

        // Clean up
        debug!("Cleaning up test entities");
        debug!("Deleting child entity with ID: {}", child_id);
        child_repo.delete(&child_id).await.expect("Failed to delete child entity");

        debug!("Deleting parent entity with ID: {}", parent_id);
        entity_repo.delete(&parent_id).await.expect("Failed to delete parent entity");

        info!("test_associated_entity_one_to_one completed successfully");
    }

    pub async fn test_associated_entity_one_to_many(
        entity_repo: impl AssociatedEntityRepository<TestEntity, TestEntityChild>,
        child_repo: impl Repository<TestEntityChild>
    ) {
        info!("Starting test_associated_entity_one_to_many");
        // Create parent entity
        let parent = TestEntity { id: None, name: "Parent Entity".to_string() };
        debug!("Created parent entity: {:?}", parent);
        let saved_parent = entity_repo.save(parent).await.expect("Failed to save parent");
        let parent_id = saved_parent.id().expect("Parent should have ID");
        info!("Saved parent entity with ID: {}", parent_id);

        // Test create_child
        let child1 = TestEntityChild { id: None, name: "Child Entity 1".to_string() };
        debug!("Creating child entity as child of parent: {:?}", child1);
        let saved_child1 = entity_repo.create_child(child1, &parent_id).await.expect("Failed to create child");
        let child1_id = saved_child1.id().expect("Child should have ID");
        info!("Created child entity with ID: {}", child1_id);

        // Test create_children
        let children = vec![
            TestEntityChild { id: None, name: "Child Entity 2".to_string() },
            TestEntityChild { id: None, name: "Child Entity 3".to_string() },
        ];
        debug!("Creating multiple children for parent");
        let saved_children = entity_repo.create_children(children, &parent_id).await.expect("Failed to create children");
        info!("Created {} child entities", saved_children.len());
        assert_eq!(saved_children.len(), 2, "Should have created 2 children");

        // Test count_children
        debug!("Counting children for parent {}", parent_id);
        let count = entity_repo.count_children(&parent_id).await.expect("Failed to count children");
        info!("Parent has {} children", count);
        assert_eq!(count, 3, "Parent should have 3 children");

        // Test find_children
        let page_req = PageRequest::new(1, 10);
        debug!("Finding children for parent {} with page request: {:?}", parent_id, page_req);
        let found_children = entity_repo.find_children(&parent_id, page_req).await.expect("Failed to find children");
        info!("Found {} children", found_children.data().len());
        assert_eq!(found_children.data().len(), 3, "Should find 3 children");

        // Test pagination of children
        let page_req = PageRequest::new(1, 2);
        debug!("Getting first page of children with size 2");
        let first_page = entity_repo.find_children(&parent_id, page_req).await.expect("Failed to get first page");
        assert_eq!(first_page.data().len(), 2, "First page should have 2 children");

        let page_req = PageRequest::new(2, 2);
        debug!("Getting second page of children with size 2");
        let second_page = entity_repo.find_children(&parent_id, page_req).await.expect("Failed to get second page");
        assert_eq!(second_page.data().len(), 1, "Second page should have 1 child");

        // Test delete_children
        debug!("Deleting all children of parent {}", parent_id);
        entity_repo.delete_children(&parent_id).await.expect("Failed to delete children");

        let count_after = entity_repo.count_children(&parent_id).await.expect("Failed to count children after deletion");
        debug!("Parent has {} children after deletion", count_after);
        assert_eq!(count_after, 0, "Parent should have 0 children after deletion");

        // Clean up
        debug!("Deleting parent entity with ID: {}", parent_id);
        entity_repo.delete(&parent_id).await.expect("Failed to delete parent entity");

        info!("test_associated_entity_one_to_many completed successfully");
    }

    pub async fn test_associated_entity_many_to_many(
        entity_repo: impl AssociatedEntityRepository<TestEntity, TestEntityChild>,
        related_repo: impl Repository<TestEntityChild>
    ) {
        info!("Starting test_associated_entity_many_to_many");
        // Create entities
        let entity1 = TestEntity { id: None, name: "Entity 1".to_string() };
        let entity2 = TestEntity { id: None, name: "Entity 2".to_string() };

        let saved_entity1 = entity_repo.save(entity1).await.expect("Failed to save entity 1");
        let entity1_id = saved_entity1.id().expect("Entity 1 should have ID");

        let saved_entity2 = entity_repo.save(entity2).await.expect("Failed to save entity 2");
        let entity2_id = saved_entity2.id().expect("Entity 2 should have ID");

        info!("Saved entities with IDs: {}, {}", entity1_id, entity2_id);

        // Create related entities
        let related1 = TestEntityChild { id: None, name: "Related 1".to_string() };
        let related2 = TestEntityChild { id: None, name: "Related 2".to_string() };
        let related3 = TestEntityChild { id: None, name: "Related 3".to_string() };

        let saved_related1 = related_repo.save(related1).await.expect("Failed to save related 1");
        let related1_id = saved_related1.id().expect("Related 1 should have ID");

        let saved_related2 = related_repo.save(related2).await.expect("Failed to save related 2");
        let related2_id = saved_related2.id().expect("Related 2 should have ID");

        let saved_related3 = related_repo.save(related3).await.expect("Failed to save related 3");
        let related3_id = saved_related3.id().expect("Related 3 should have ID");

        info!("Saved related entities with IDs: {}, {}, {}", related1_id, related2_id, related3_id);

        // Test create_associated
        debug!("Creating a new associated entity for entity1");
        let new_related = TestEntityChild { id: None, name: "New Related".to_string() };
        let saved_new_related = entity_repo.create_associated(&entity1_id, new_related).await
            .expect("Failed to create associated entity");
        let new_related_id = saved_new_related.id().expect("New related entity should have ID");
        info!("Created new associated entity with ID: {}", new_related_id);

        // Test associate
        debug!("Associating entity1 with related1 and related2");
        entity_repo.associate(&entity1_id, &related1_id).await.expect("Failed to associate entity1 with related1");
        entity_repo.associate(&entity1_id, &related2_id).await.expect("Failed to associate entity1 with related2");

        debug!("Associating entity2 with related2 and related3");
        entity_repo.associate(&entity2_id, &related2_id).await.expect("Failed to associate entity2 with related2");
        entity_repo.associate(&entity2_id, &related3_id).await.expect("Failed to associate entity2 with related3");

        // Test is_associated
        debug!("Checking associations");
        assert!(entity_repo.is_associated(&entity1_id, &related1_id).await.expect("Failed to check association"),
            "entity1 should be associated with related1");
        assert!(entity_repo.is_associated(&entity1_id, &related2_id).await.expect("Failed to check association"),
            "entity1 should be associated with related2");
        assert!(entity_repo.is_associated(&entity2_id, &related2_id).await.expect("Failed to check association"),
            "entity2 should be associated with related2");
        assert!(entity_repo.is_associated(&entity2_id, &related3_id).await.expect("Failed to check association"),
            "entity2 should be associated with related3");
        assert!(!entity_repo.is_associated(&entity1_id, &related3_id).await.expect("Failed to check association"),
            "entity1 should not be associated with related3");

        // Test count_associated
        debug!("Counting associated entities for entity1");
        let entity1_count = entity_repo.count_associated(&entity1_id).await.expect("Failed to count associated entities");
        info!("entity1 has {} associated entities", entity1_count);
        assert_eq!(entity1_count, 3, "entity1 should have 3 associations (related1, related2, and new_related)");

        // Test find_associated
        let page_req = PageRequest::new(1, 10);
        debug!("Finding associated entities for entity1");
        let entity1_associated = entity_repo.find_associated(&entity1_id, page_req.clone()).await.expect("Failed to find associated entities");
        info!("Found {} associated entities for entity1", entity1_associated.data().len());
        assert_eq!(entity1_associated.data().len(), 3, "Should find 3 associated entities for entity1");

        // Test find_associated_to
        debug!("Finding entities associated to related2");
        let related2_associated = entity_repo.find_associated_to(&related2_id, page_req).await.expect("Failed to find entities associated to related2");
        info!("Found {} entities associated to related2", related2_associated.data().len());
        assert_eq!(related2_associated.data().len(), 2, "Should find 2 entities associated to related2");

        // Test dissociate
        debug!("Dissociating entity1 from related2");
        entity_repo.dissociate(&entity1_id, &related2_id).await.expect("Failed to dissociate entity1 from related2");

        assert!(!entity_repo.is_associated(&entity1_id, &related2_id).await.expect("Failed to check association after dissociation"),
            "entity1 should no longer be associated with related2");
        assert!(entity_repo.is_associated(&entity2_id, &related2_id).await.expect("Failed to check association"),
            "entity2 should still be associated with related2");

        // Test dissociate_all
        debug!("Dissociating entity1 from all related entities");
        entity_repo.dissociate_all(&entity1_id).await.expect("Failed to dissociate entity1 from all");

        let count_after = entity_repo.count_associated(&entity1_id).await.expect("Failed to count associated entities after dissociate_all");
        assert_eq!(count_after, 0, "entity1 should have 0 associated entities after dissociate_all");

        // Test dissociate_from_all
        debug!("Dissociating related3 from all entities");
        entity_repo.dissociate_from_all(&related3_id).await.expect("Failed to dissociate related3 from all");

        assert!(!entity_repo.is_associated(&entity2_id, &related3_id).await.expect("Failed to check association after dissociate_from_all"),
            "entity2 should no longer be associated with related3");

        // Clean up
        debug!("Cleaning up test entities");
        entity_repo.delete(&entity1_id).await.expect("Failed to delete entity1");
        entity_repo.delete(&entity2_id).await.expect("Failed to delete entity2");

        related_repo.delete(&related1_id).await.expect("Failed to delete related1");
        related_repo.delete(&related2_id).await.expect("Failed to delete related2");
        related_repo.delete(&related3_id).await.expect("Failed to delete related3");
        related_repo.delete(&new_related_id).await.expect("Failed to delete new_related");

        info!("test_associated_entity_many_to_many completed successfully");
    }

    // pub async fn test_throughput_repository(repo: impl ThroughputRepository<TestEntity>) {
    //     // Create test entities
    //     let entities = vec![
    //         TestEntity { id: None, name: "Stream Entity 1".to_string() },
    //         TestEntity { id: None, name: "Stream Entity 2".to_string() },
    //         TestEntity { id: None, name: "Stream Entity 3".to_string() },
    //     ];
    //
    //     let mut saved_ids = Vec::new();
    //     for entity in entities {
    //         let saved = repo.save(entity).await.expect("Failed to save entity");
    //         if let Some(id) = saved.id() {
    //             saved_ids.push(id);
    //         }
    //     }
    //
    //     // Test stream
    //     let mut stream = repo.stream().await.expect("Failed to get stream");
    //     let mut count = 0;
    //
    //     while let Some(result) = stream.next().await {
    //         let entity = result.expect("Stream item should be valid");
    //         assert!(!entity.name.is_empty(), "Entity name should not be empty");
    //         count += 1;
    //     }
    //
    //     assert!(count >= 3, "Stream should contain at least 3 entities");
    //
    //     // Clean up
    //     for id in saved_ids {
    //         repo.delete(&id).await.expect("Failed to delete entity");
    //     }
    // }

    // pub async fn test_live_repository(repo: impl LiveRepository<TestEntity>) {
    //     // Create a test entity
    //     let entity = TestEntity {
    //         id: None,
    //         name: "Live Entity".to_string(),
    //     };
    //
    //     let saved_entity = repo.save(entity).await.expect("Failed to save entity");
    //     let id = saved_entity.id().expect("Entity should have ID");
    //
    //     // Test live_single
    //     let mut live_stream = repo.live_single(&id).await.expect("Failed to get live stream");
    //
    //     // Update the entity to trigger the live stream
    //     let mut updated_entity = saved_entity.clone();
    //     updated_entity.name = "Updated Live Entity".to_string();
    //     repo.save(updated_entity).await.expect("Failed to update entity");
    //
    //     // Test live_table
    //     let mut table_stream = repo.live_table().await.expect("Failed to get table stream");
    //
    //     // Create another entity to trigger the table stream
    //     let another_entity = TestEntity {
    //         id: None,
    //         name: "Another Live Entity".to_string(),
    //     };
    //     let another_saved = repo.save(another_entity).await.expect("Failed to save another entity");
    //     let another_id = another_saved.id().expect("Entity should have ID");
    //
    //     // Clean up
    //     repo.delete(&id).await.expect("Failed to delete entity");
    //     repo.delete(&another_id).await.expect("Failed to delete another entity");
    // }
}
