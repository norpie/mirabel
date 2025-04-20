pub(crate) mod surrealdb;

#[cfg(test)]
pub mod tests {
    use backend_derive::{named_struct};
    use serde::{Deserialize, Serialize};
    use surrealdb::sql::Thing;
    use futures::StreamExt;

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
        // Create a test entity
        let entity = TestEntity {
            id: None,
            name: "Test Entity".to_string(),
        };

        // Test save
        let saved_entity = repo.save(entity.clone()).await.expect("Failed to save entity");
        assert!(saved_entity.id().is_some(), "Saved entity should have an ID");

        let id = saved_entity.id().unwrap();

        // Test find
        let found_entity = repo.find(&id).await.expect("Failed to find entity");
        assert!(found_entity.is_some(), "Entity should be found");
        let found_entity = found_entity.unwrap();
        assert_eq!(found_entity.name, saved_entity.name, "Found entity should match saved entity");

        // Test exists
        let exists = repo.exists(&id).await.expect("Failed to check existence");
        assert!(exists, "Entity should exist");

        // Test count
        let count = repo.count().await.expect("Failed to count entities");
        assert!(count > 0, "Count should be greater than 0");

        // Test delete
        repo.delete(&id).await.expect("Failed to delete entity");
        let exists_after_delete = repo.exists(&id).await.expect("Failed to check existence");
        assert!(!exists_after_delete, "Entity should not exist after deletion");
    }

    pub async fn test_field_searchable_repository(repo: impl FieldSearchableRepository<TestEntity>) {
        // Create test entities
        let entity1 = TestEntity {
            id: None,
            name: "Unique Test Entity".to_string(),
        };
        let entity2 = TestEntity {
            id: None,
            name: "Another Test Entity".to_string(),
        };

        let saved1 = repo.save(entity1).await.expect("Failed to save entity 1");
        let saved2 = repo.save(entity2).await.expect("Failed to save entity 2");

        // Test search
        let page_req = PageRequest::new(1, 10); // Changed from 0 to 1
        let search_results = repo.search(&["name"], "Unique", page_req).await.expect("Failed to search");

        assert_eq!(search_results.data().len(), 1, "Search should return one result");
        assert_eq!(search_results.data()[0].name, saved1.name, "Search result should match");

        // Clean up
        if let Some(id) = saved1.id() {
            repo.delete(&id).await.expect("Failed to delete entity 1");
        }
        if let Some(id) = saved2.id() {
            repo.delete(&id).await.expect("Failed to delete entity 2");
        }
    }

    pub async fn test_field_findable_repository(repo: impl FieldFindableRepository<TestEntity>) {
        // Create test entities
        let entity1 = TestEntity {
            id: None,
            name: "FindableEntity".to_string(),
        };
        let entity2 = TestEntity {
            id: None,
            name: "OtherEntity".to_string(),
        };

        let saved1 = repo.save(entity1).await.expect("Failed to save entity 1");
        let saved2 = repo.save(entity2).await.expect("Failed to save entity 2");

        // Test find by field
        let page_req = PageRequest::new(1, 10); // Changed from 0 to 1
        let find_results = repo.find_by_field("name", "FindableEntity", page_req).await
            .expect("Failed to find by field");

        assert_eq!(find_results.data().len(), 1, "Find by field should return one result");
        assert_eq!(find_results.data()[0].name, saved1.name, "Found entity should match");

        // Test exists by field
        let exists = repo.exists_by_field("name", "FindableEntity").await
            .expect("Failed to check existence by field");
        assert!(exists, "Entity should exist by field");

        let nonexistent = repo.exists_by_field("name", "NonexistentEntity").await
            .expect("Failed to check existence by field");
        assert!(!nonexistent, "Nonexistent entity should not exist");

        // Clean up
        if let Some(id) = saved1.id() {
            repo.delete(&id).await.expect("Failed to delete entity 1");
        }
        if let Some(id) = saved2.id() {
            repo.delete(&id).await.expect("Failed to delete entity 2");
        }
    }

    pub async fn test_public_entity_repository(repo: impl PublicEntityRepository<TestEntity>) {
        // Create multiple test entities
        let entities = vec![
            TestEntity { id: None, name: "Public Entity 1".to_string() },
            TestEntity { id: None, name: "Public Entity 2".to_string() },
            TestEntity { id: None, name: "Public Entity 3".to_string() },
        ];

        let mut saved_ids = Vec::new();
        for entity in entities {
            let saved = repo.save(entity).await.expect("Failed to save entity");
            if let Some(id) = saved.id() {
                saved_ids.push(id);
            }
        }

        // Test find_all
        let page_req = PageRequest::new(1, 10); // Changed from 0 to 1
        let all_entities = repo.find_all(page_req).await.expect("Failed to find all entities");

        assert!(all_entities.data().len() >= 3, "Should find at least 3 entities");

        // Test pagination
        let page_req = PageRequest::new(1, 2); // Changed from 0 to 1
        let first_page = repo.find_all(page_req).await.expect("Failed to get first page");
        assert_eq!(first_page.data().len(), 2, "First page should have 2 items");

        let page_req = PageRequest::new(2, 2); // Changed from 1 to 2
        let second_page = repo.find_all(page_req).await.expect("Failed to get second page");
        assert!(!second_page.data().is_empty(), "Second page should have at least 1 item");

        // Clean up
        for id in saved_ids {
            repo.delete(&id).await.expect("Failed to delete entity");
        }
    }

    pub async fn test_associated_entity_repository(
        entity_repo: impl AssociatedEntityRepository<TestEntity, TestEntityChild>,
        child_repo: impl Repository<TestEntityChild>
    ) {
        // Create parent and child entities
        let parent = TestEntity { id: None, name: "Parent Entity".to_string() };
        let saved_parent = entity_repo.save(parent).await.expect("Failed to save parent");
        let parent_id = saved_parent.id().expect("Parent should have ID");

        let child = TestEntityChild { id: None, name: "Child Entity".to_string() };
        let saved_child = child_repo.save(child).await.expect("Failed to save child");
        let child_id = saved_child.id().expect("Child should have ID");

        // Test one-to-one relationship methods
        entity_repo.relate(&parent_id, &child_id).await.expect("Failed to relate entities");

        let related = entity_repo.find_related(&child_id).await.expect("Failed to find related");
        assert!(related.is_some(), "Related entity should exist");

        let exists_related = entity_repo.exists_related(&child_id).await.expect("Failed to check existence");
        assert!(exists_related, "Related entity should exist");

        // Test one-to-many relationship methods
        let new_child = TestEntity { id: None, name: "Child of Parent".to_string() };
        let saved_new_child = entity_repo.create_child(new_child, &child_id).await
            .expect("Failed to create child");

        let page_req = PageRequest::new(1, 10); // Changed from 0 to 1
        let children = entity_repo.find_children(&child_id, page_req.clone()).await
            .expect("Failed to find children");

        assert!(!children.data().is_empty(), "Should find at least one child");

        let count = entity_repo.count_children(&child_id).await.expect("Failed to count children");
        assert!(count > 0, "Child count should be greater than 0");

        // Test many-to-many relationship methods
        let another_entity = TestEntity { id: None, name: "Associated Entity".to_string() };
        let saved_another = entity_repo.save(another_entity).await.expect("Failed to save another entity");
        let another_id = saved_another.id().expect("Entity should have ID");

        entity_repo.associate(&another_id, &child_id).await.expect("Failed to associate entities");

        let is_associated = entity_repo.is_associated(&another_id, &child_id).await
            .expect("Failed to check association");
        assert!(is_associated, "Entities should be associated");

        let associated = entity_repo.find_associated(&child_id, page_req.clone()).await
            .expect("Failed to find associated entities");
        assert!(!associated.data().is_empty(), "Should find at least one associated entity");

        let associated_to = entity_repo.find_associated_to(&another_id, page_req.clone()).await
            .expect("Failed to find entities associated to");
        assert!(!associated_to.data().is_empty(), "Should find at least one entity associated to");

        // Clean up associations
        entity_repo.dissociate(&another_id, &child_id).await.expect("Failed to dissociate entities");
        let dissociate_count = entity_repo.dissociate_all(&parent_id).await
            .expect("Failed to dissociate all");

        // Delete test data
        entity_repo.delete_children(&child_id).await.expect("Failed to delete children");
        entity_repo.delete(&parent_id).await.expect("Failed to delete parent");
        entity_repo.delete(&another_id).await.expect("Failed to delete another entity");
        entity_repo.delete(&saved_new_child.id().unwrap()).await.expect("Failed to delete new child");
        child_repo.delete(&child_id).await.expect("Failed to delete child");
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
