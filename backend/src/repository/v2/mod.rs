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
