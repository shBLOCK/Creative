using System;
using System.Collections;
using System.Collections.Generic;
using EzySlice;
using Unity.Mathematics;
using UnityEngine;
using Object = UnityEngine.Object;
using Random = UnityEngine.Random;

public class CakeTrap : MonoBehaviour {
    [SerializeField] private Transform player;
    [SerializeField] private GameObject door;

    private Material _material;
    private Vector3 _baseScale;

    public bool exploding = false;
    private float fuseTime = 2.5f;

    private void Awake() {
        _material = GetComponent<MeshRenderer>().material;
        _baseScale = transform.localScale;
    }

    private void Update() {
        if ((player.position - transform.position).magnitude < 2.5f) {
            exploding = true;
        }

        if (exploding) {
            _material.SetColor("_EmissionColor", Color.white * math.floor(fuseTime % 0.5f * 4f) * 3f);
            transform.localScale = _baseScale * (math.exp(-fuseTime * 10f) * 0.5f + 1f);
            if (fuseTime <= 0f) {
                GetComponent<MeshRenderer>().enabled = false;
                enabled = false;
                explodeDoor();
                transform.GetChild(0).gameObject.SetActive(true);
            }

            fuseTime -= Time.deltaTime;
        }
    }

    private void explodeDoor() {
        var parent = door.transform.parent;
        
        var objs = new LinkedList<GameObject>();
        objs.AddFirst(door);

        // var prefab = new GameObject(
        //     "Fragments",
        //     typeof(MeshFilter),
        //     typeof(MeshRenderer),
        //     typeof(MeshCollider),
        //     typeof(Rigidbody)
        // );
        // prefab.GetComponent<MeshFilter>().sharedMesh = new Mesh();

        for (int i = 0; i < 100; i++) {
            if (objs.Count >= 15) break;

            var obj = objs.First.Value;

            var newObjs = obj.SliceInstantiate(
                obj.transform.position + Random.insideUnitSphere * 1f,
                Random.insideUnitSphere.normalized,
                obj.GetComponent<MeshRenderer>().material
            );
            if (newObjs == null) continue;
            foreach (var newObj in newObjs) {
                objs.AddLast(newObj);
            }

            objs.RemoveFirst();
            Destroy(obj);
        }

        foreach (var obj in objs) {
            obj.transform.SetParent(parent, false);
            var meshFilter = obj.GetComponent<MeshFilter>();
            var collider = obj.AddComponent<MeshCollider>();
            collider.sharedMesh = meshFilter.mesh;
            collider.convex = true;
            var rigidBody = obj.AddComponent<Rigidbody>();
            rigidBody.mass = 0.01f;
            rigidBody.AddForce(
                (rigidBody.worldCenterOfMass - transform.position).normalized * 15f + Random.insideUnitSphere * 3f,
                ForceMode.VelocityChange
            );
        }
    }
}