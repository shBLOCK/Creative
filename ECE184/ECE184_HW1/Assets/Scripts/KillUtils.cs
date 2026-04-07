using System.Collections;
using System.Collections.Generic;
using EzySlice;
using UnityEngine;
using Plane = EzySlice.Plane;

public static class KillUtils {
    public static void Kill(GameObject gameObject) {
        var orgTransform = gameObject.transform;

        var objs = new LinkedList<GameObject>();
        objs.AddFirst(gameObject);

        var prefab = new GameObject(
            "Fragments",
            typeof(MeshFilter),
            typeof(MeshRenderer),
            typeof(MeshCollider),
            typeof(Rigidbody)
        );
        prefab.GetComponent<MeshFilter>().sharedMesh = new Mesh();

        for (int i = 0; i < 100; i++) {
            if (objs.Count >= 15) break;

            var obj = objs.First.Value;

            var newObjs = obj.SliceInstantiate(
                obj.transform.position + Random.insideUnitSphere * 0.5f,
                Random.insideUnitSphere.normalized,
                obj.GetComponent<MeshRenderer>().material
            );
            if (newObjs == null) continue;
            foreach (var newObj in newObjs) {
                objs.AddLast(newObj);
            }

            objs.RemoveFirst();
            Object.Destroy(obj);
        }

        foreach (var obj in objs) {
            var meshFilter = obj.GetComponent<MeshFilter>();
            var collider = obj.AddComponent<MeshCollider>();
            collider.sharedMesh = meshFilter.mesh;
            collider.convex = true;
            var rigidBody = obj.AddComponent<Rigidbody>();
            rigidBody.mass = 0.01f;
            rigidBody.AddForce(Random.insideUnitSphere * 7f, ForceMode.VelocityChange);
        }
    }
}