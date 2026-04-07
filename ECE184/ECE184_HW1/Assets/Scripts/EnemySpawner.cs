using UnityEngine;
using Random = UnityEngine.Random;

public class EnemySpawner : MonoBehaviour {
    [SerializeField] private GameObject enemyPrefab;
    [SerializeField] private float spawnInterval = 2f;
    private float spawnTimer = 0f;

    private void FixedUpdate() {
        spawnTimer += Time.fixedDeltaTime;
        for (; spawnTimer >= spawnInterval; spawnTimer -= spawnInterval) {
            var spawnLocation = Random.insideUnitCircle.normalized * 13f;
            Instantiate(
                enemyPrefab,
                transform.position + new Vector3(spawnLocation.x, 0f, spawnLocation.y),
                Quaternion.identity
            );
        }
    }
}