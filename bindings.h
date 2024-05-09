/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

typedef enum capi_gender {
  boy,
  girl,
} capi_gender;

typedef struct capi_student {
  int num;
  int total;
  char name[20];
  float scores[3];
  enum capi_gender gender;
} capi_student;

struct capi_student *student_new(void);

struct capi_student *student_alice(void);

void student_free(struct capi_student *p_stu);
