#!/bin/bash
set -e

until mc alias set local ${S3_API_URL} "${MINIO_ROOT_USER}" "${MINIO_ROOT_PASSWORD}" >/dev/null 2>&1; do
  sleep 1
done
echo "Minio alias set."

if mc ls local/"${S3_BUCKET}" >/dev/null 2>&1; then
  echo "Bucket '"${S3_BUCKET}"' already exists — skipping init."
  exit 0
fi

mc mb local/"${S3_BUCKET}"
echo "Created bucket '"${S3_BUCKET}"'."

cat >/tmp/"${S3_BUCKET}"-rw.json <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    { "Effect": "Allow",
      "Action": ["s3:GetBucketLocation","s3:ListBucket"],
      "Resource": ["arn:aws:s3:::$S3_BUCKET"]
    },
    { "Effect": "Allow",
      "Action": ["s3:GetObject","s3:PutObject","s3:DeleteObject"],
      "Resource": ["arn:aws:s3:::$S3_BUCKET/*"]
    }
  ]
}
EOF

mc admin policy create local "${S3_BUCKET}"-rw /tmp/"${S3_BUCKET}"-rw.json
echo "Policy created."

mc admin accesskey create local --access-key "$S3_ACCESS_KEY" --secret-key "$S3_SECRET_KEY"
echo "Access key '$S3_ACCESS_KEY' created."

mc admin policy attach local --user "${S3_ACCESS_KEY}" "${S3_BUCKET}"-rw
echo "Policy attached."

echo "Init complete."