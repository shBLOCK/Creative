using UnityEngine;

public class EnemySpawner : MonoBehaviour {
    [SerializeField] private GameObject enemyPrefab;

    [SerializeField] private float spawnInterval = 5f;
    private float spawnProgress = 1f;

    void Update() {
        spawnProgress += (1f / spawnInterval) * Time.deltaTime;
        if (spawnProgress >= 1f) {
            spawnProgress = 0f;
            var offset = Random.insideUnitSphere * 8.5f;
            offset.y = 0;
            Instantiate(enemyPrefab, transform.position + offset, Quaternion.Euler(0f, Random.Range(0f, 360f), 0f));
        }
    }
}