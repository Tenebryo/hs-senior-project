#include <jni.h>       /* where everything is defined */
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>

JavaVM *jvm = NULL;     /* denotes a Java VM */
JNIEnv *env = NULL;     /* pointer to native method interface */
jclass cls;             /* Class ID of the Search class*/
jobject obj;            /* Object ID of the Search class instance*/

jmethodID mids[16];

const int FN_SOLUTION           = 0;
const int FN_NEXT               = 1;
const int FN_IS_INITED          = 2;
const int FN_NUMBER_OF_PROBES   = 3;
const int FN_LENGTH             = 4;
const int FN_INIT               = 5;

int rs_java_vm_create() {
    printf("INITIALIZING THE JVM\n");
    JavaVMInitArgs vm_args; /* JDK/JRE 6 VM initialization arguments */
    JavaVMOption* options = (JavaVMOption*)malloc(2*sizeof(JavaVMOption));
    options[0].optionString = (char*)"-Djava.class.path=./min2phase/";
    options[1].optionString = (char*)"-Djava.library.path=.";
    vm_args.version = JNI_VERSION_1_6;
    vm_args.nOptions = 2;
    vm_args.options = options;
    vm_args.ignoreUnrecognized = false;
    /* load and initialize a Java VM, return a JNI interface
     * pointer in env */
     
    jint t = JNI_CreateJavaVM(&jvm, (void**)&env, (void**)&vm_args);
    free(options);
    
    if (t != JNI_OK) {
        printf("ERROR INITIALIZING JAVA\n");
        return -1;
    }
    
    cls = (*env)->FindClass(env, "cs/min2phase/Search");
    
    if (!cls) {
        printf("Error finding the Search class.\n");
        return -1;
    }
    printf("CLS: %16i\n", cls);
    
    jmethodID mid = (*env)->GetMethodID(env, cls, "<init>", "()V");
    
    if (!mid) {
        printf("Error finding the constructor for the Search class\n");
        return -1;
    }
    printf("CTR: %16i\n", mid);
    
    /*get all the method Ids of public methods of the Search class*/
    mids[FN_SOLUTION        ] = (*env)->GetMethodID(env, cls, "solution",       "(Ljava/lang/String;IJJI)Ljava/lang/String;");
    mids[FN_NEXT            ] = (*env)->GetMethodID(env, cls, "next",           "(JJI)Ljava/lang/String;");
    mids[FN_IS_INITED       ] = (*env)->GetStaticMethodID(env, cls, "isInited", "()Z");
    mids[FN_NUMBER_OF_PROBES] = (*env)->GetMethodID(env, cls, "numberOfProbes", "()J");
    mids[FN_LENGTH          ] = (*env)->GetMethodID(env, cls, "length",         "()I");
    mids[FN_INIT            ] = (*env)->GetStaticMethodID(env, cls, "init",     "()V");

    for (int i = 0; i < 6; i++) {
        printf("MID: %16i\n",mids[i]);
    }
    
    obj = (*env)->NewObject(env, cls, mid);
    
    if (!obj) {
        printf("Error creating instance of the Search class\n");
        return -1;
    }
    printf("OBJ: %16i\n", obj);
    
    printf("DONE INITIALIZING THE JVM\n");
    return 0;
}

int rs_java_vm_destroy() {
    /* We are done. */
    printf("CLEANING UP THE JVM\n");
    return (*jvm)->DestroyJavaVM(jvm);
}

//Other Functions
void solution(const char* facelets, int max_depth, long long probe_max, long long probe_min, int verbose, char *soln, size_t *soln_len) {
    jmethodID mid;
    if (mid=mids[FN_SOLUTION]) {
        printf("SEARCHING SOLUTION\n");
        jstring jfacelets = (*env)->NewStringUTF(env, facelets);
        
        jstring solution_manuever = (*env)->CallObjectMethod(env, obj, mid, 
            jfacelets, 
            (jint)max_depth,
            (jlong)probe_max,
            (jlong)probe_min,
            (jint)verbose
        );
        
        jint t = (*env)->GetStringUTFLength(env, solution_manuever);
        *soln_len = (int)t;
        
        jboolean test;
        char *s = (*env)->GetStringUTFChars(env, solution_manuever, &test);
        memcpy(soln, s, t);
        printf("FINISHED SEARCHING\n");
        return;
    }
    *soln_len = 0;
    printf("ERROR LOADING FUNCTION: SOLUTION\n");
}

void next(long long probe_max, long long probe_min, int verbose, char *soln, size_t *soln_len) {
    jmethodID mid;
    if (mid=mids[FN_NEXT]) {
        
        jstring solution_manuever = (*env)->CallObjectMethod(env, obj, mid, 
            (jlong)probe_max,
            (jlong)probe_min,
            (jint)verbose
        );
        
        *soln_len = (*env)->GetStringUTFLength(env, solution_manuever);
        (*env)->GetStringUTFChars(env, solution_manuever, soln);
    }
    *soln_len = 0;
}

int is_inited() {
    jmethodID mid;
    if (mid=mids[FN_IS_INITED]) {
        
        jboolean r = (*env)->CallStaticBooleanMethod(env, cls, mid);
        
        return (r?1:0);
    }
    return -1;
}

long long number_of_probes() {
    jmethodID mid;
    if (mid=mids[FN_NUMBER_OF_PROBES]) {
        
        jlong r = (*env)->CallLongMethod(env, obj, mid);
        
        return r;
    }
    return -1;
}

int length() {
    jmethodID mid;
    if (mid=mids[FN_NUMBER_OF_PROBES]) {
        
        jint r = (*env)->CallIntMethod(env, obj, mid);
        
        return r;
    }
    return -1;
}

void init() {
    jmethodID mid;
    if (mid=mids[FN_INIT]) {
        
        printf("INITIALIZING SOLVER\n");
        
        (*env)->CallStaticVoidMethod(env, cls, mid);
        
        printf("DONE INITIALIZING SOLVER\n");
        
    }
}

