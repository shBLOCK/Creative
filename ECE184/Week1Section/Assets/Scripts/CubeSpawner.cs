using System;
using UnityEngine;
using Random = UnityEngine.Random;

public class CubeSpawner : MonoBehaviour {
    [SerializeField] private GameObject cubePrefab;
    [SerializeField] private float spawnRadius = 10f;
    
    public void SpawnCube() {
        Instantiate(cubePrefab, transform.position + Random.insideUnitSphere * spawnRadius, Random.rotation);
    }

    private void OnDrawGizmosSelected() {
        Gizmos.DrawWireSphere(transform.position, spawnRadius);
    }
}