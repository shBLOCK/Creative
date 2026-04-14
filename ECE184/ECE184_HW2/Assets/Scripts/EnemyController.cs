using System;
using System.Collections;
using System.Linq;
using JetBrains.Annotations;
using UnityEngine;
using UnityEngine.AI;
using Random = UnityEngine.Random;

public class EnemyController : MonoBehaviour {
    [SerializeField] private float patrolRadius = 20f;
    [SerializeField] private float aggroRadius = 10f;
    [SerializeField] private float attackRadius = 3f;
    
    [SerializeField] private GameObject deathParticle;

    private Animator _animator;
    private NavMeshAgent _navMeshAgent;
    private Health _health;

    private enum State {
        Patrol,
        Aggro,
        Dead
    }

    private State _state = State.Patrol;
    private GameObject _aggroTarget;
    private Coroutine _patrolCoroutine;

    private void Awake() {
        _animator = GetComponent<Animator>();
        _navMeshAgent = GetComponent<NavMeshAgent>();
        _health = GetComponent<Health>();
    }

    private void Start() {
        _patrolCoroutine = StartCoroutine(PatrolCoroutine());
    }

    private void Update() {
        if (_state != State.Dead && _health.health <= 0) {
            _state = State.Dead;
            _animator.SetTrigger("Death");
            _aggroTarget.GetComponent<PlayerController>().killCount++;
            deathParticle.SetActive(true);
        }
        
        if (_state != State.Dead) {
            _animator.SetFloat("Speed", _navMeshAgent.velocity.magnitude / _navMeshAgent.speed);
        }

        switch (_state) {
            case State.Patrol:
                _aggroTarget = FindAggroTarget();
                if (_aggroTarget) {
                    _state = State.Aggro;
                    StopCoroutine(_patrolCoroutine);
                }

                break;
            case State.Aggro:
                _navMeshAgent.SetDestination(_aggroTarget.transform.position);
                if ((transform.position - _aggroTarget.transform.position).magnitude <= attackRadius) {
                    _animator.SetTrigger("Attack");
                }

                if (_aggroTarget.GetComponent<Health>().health <= 0) {
                    _animator.ResetTrigger("Attack");
                    _aggroTarget = null;
                    _state = State.Patrol;
                    _patrolCoroutine = StartCoroutine(PatrolCoroutine());
                }

                break;
            case State.Dead:
                break;
            default:
                throw new ArgumentOutOfRangeException();
        }
    }

    private IEnumerator PatrolCoroutine() {
        while (true) {
            var offset = Random.insideUnitSphere * patrolRadius;
            offset.y = 0;
            _navMeshAgent.SetDestination(offset);
            yield return new WaitForSeconds(Random.Range(3f, 5f));
        }
    }

    [CanBeNull]
    private GameObject FindAggroTarget() {
        return Physics.OverlapSphere(transform.position, aggroRadius).FirstOrDefault(it => it.CompareTag("Player"))
            ?.gameObject;
    }

    private void animDoAttack() {
        if (_aggroTarget.TryGetComponent<Health>(out var health)) {
            health.Damage(1);
        }
    }

    private void animResetAttack() { }
}