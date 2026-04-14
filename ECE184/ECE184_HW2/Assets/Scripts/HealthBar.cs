using System;
using Unity.Mathematics;
using UnityEngine;

public class HealthBar : MonoBehaviour {
    private GameObject canvas;
    [SerializeField] private GameObject healthBarPrefab;
    [SerializeField] private Transform anchor;

    private Health _health;
    private GameObject healthBar;
    private RectTransform healthBarRectTransform;

    void Start() {
        canvas = GameObject.Find("WorldCanvas");
        _health = GetComponent<Health>();
        healthBar = Instantiate(healthBarPrefab, canvas.transform);
        healthBarRectTransform = healthBar.transform.GetChild(0).gameObject.GetComponent<RectTransform>();
    }

    private void LateUpdate() {
        healthBar.transform.position = anchor.position;
        healthBar.transform.forward = -Camera.main.transform.forward;
        healthBarRectTransform.localScale = new Vector3(math.clamp(_health.health / _health.initialHealth, 0f, 1f), 1f, 1f);
    }
}